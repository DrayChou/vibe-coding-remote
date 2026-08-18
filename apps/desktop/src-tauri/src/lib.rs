use qrcode::{QrCode, render::svg};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{
    Manager, State,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
use tokio::sync::{Mutex, oneshot};
use uuid::Uuid;
use vibe_coding_remote_server::RuntimeOptions;

#[cfg(unix)]
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

const DEFAULT_PORT: u16 = 8765;

#[derive(Debug, Clone, Deserialize)]
struct DesktopConfig {
    host: IpAddr,
    port: u16,
    auth_token: String,
}

#[derive(Debug, Clone, Serialize)]
struct DesktopStatus {
    running: bool,
    last_error: Option<String>,
    host: String,
    port: u16,
    config_path: String,
    platform: &'static str,
    input_permission: &'static str,
    lan_urls: Vec<String>,
    token_present: bool,
    mobile_web_ready: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ConnectionConfig {
    endpoint: String,
    token: String,
    import_url: String,
}

#[derive(Default)]
struct ServerRuntime {
    running: bool,
    last_error: Option<String>,
    shutdown: Option<oneshot::Sender<()>>,
}

#[derive(Clone)]
struct DesktopState {
    config_path: PathBuf,
    mobile_web_dir: PathBuf,
    runtime: Arc<Mutex<ServerRuntime>>,
}

#[tauri::command]
async fn get_desktop_status(state: State<'_, DesktopState>) -> Result<DesktopStatus, String> {
    let config = load_or_create_config(&state.config_path)?;
    let runtime = state.runtime.lock().await;

    Ok(DesktopStatus {
        running: runtime.running,
        last_error: runtime.last_error.clone(),
        host: config.host.to_string(),
        port: config.port,
        config_path: state.config_path.display().to_string(),
        platform: std::env::consts::OS,
        input_permission: input_permission_status(),
        lan_urls: lan_urls(config.host, config.port),
        token_present: !config.auth_token.is_empty(),
        mobile_web_ready: state.mobile_web_dir.join("index.html").is_file(),
    })
}

#[tauri::command]
fn get_connection_config(state: State<'_, DesktopState>) -> Result<ConnectionConfig, String> {
    let config = load_or_create_config(&state.config_path)?;
    let endpoint = lan_urls(config.host, config.port)
        .into_iter()
        .next()
        .unwrap_or_else(|| format!("http://127.0.0.1:{}", config.port));
    let import_url =
        vibe_coding_remote_server::create_import_url(endpoint.clone(), config.auth_token.clone())
            .map_err(|error| error.to_string())?;
    Ok(ConnectionConfig {
        endpoint,
        token: config.auth_token,
        import_url,
    })
}

#[tauri::command]
fn get_connection_qr_svg(state: State<'_, DesktopState>) -> Result<String, String> {
    let config = get_connection_config(state)?;
    QrCode::new(config.import_url.as_bytes())
        .map(|code| {
            code.render::<svg::Color>()
                .min_dimensions(240, 240)
                .dark_color(svg::Color("#111318"))
                .light_color(svg::Color("#ffffff"))
                .build()
        })
        .map_err(|error| format!("failed to generate connection QR code: {error}"))
}

#[tauri::command]
async fn save_and_restart_server(
    host: String,
    port: u16,
    state: State<'_, DesktopState>,
) -> Result<(), String> {
    if !(1024..=65535).contains(&port) {
        return Err("port must be between 1024 and 65535".to_owned());
    }
    let host = host
        .parse::<IpAddr>()
        .map_err(|error| format!("invalid listen address: {error}"))?;
    if host != IpAddr::V4(Ipv4Addr::LOCALHOST) && host != IpAddr::V4(Ipv4Addr::UNSPECIFIED) {
        return Err("listen address must be 127.0.0.1 or 0.0.0.0".to_owned());
    }

    let mut config = load_or_create_config(&state.config_path)?;
    config.host = host;
    config.port = port;
    write_config(&state.config_path, &config)?;
    restart_server(state.inner().clone()).await
}

#[tauri::command]
fn request_input_permission() -> bool {
    #[cfg(target_os = "macos")]
    {
        vibe_coding_remote_server::request_desktop_input_permission()
    }

    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}

pub fn run() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                tracing_subscriber::EnvFilter::new("vibe_coding_remote_server=info")
            }),
        )
        .with_target(false)
        .compact()
        .try_init();

    tauri::Builder::default()
        .setup(|app| {
            let config_dir = app
                .path()
                .app_config_dir()
                .map_err(|error| format!("failed to resolve app config directory: {error}"))?;
            let state = DesktopState {
                config_path: config_dir.join("config.toml"),
                mobile_web_dir: resolve_mobile_web_dir(app)?,
                runtime: Arc::new(Mutex::new(ServerRuntime::default())),
            };
            load_or_create_config(&state.config_path)?;
            setup_tray(app)?;
            app.manage(state.clone());
            tauri::async_runtime::spawn(async move {
                if let Err(error) = start_server(state.clone()).await {
                    let mut runtime = state.runtime.lock().await;
                    runtime.running = false;
                    runtime.last_error = Some(error);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_desktop_status,
            get_connection_config,
            get_connection_qr_svg,
            save_and_restart_server,
            request_input_permission
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Vibe Coding Remote desktop application");
}

fn setup_tray(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let open_item = MenuItem::with_id(app, "open", "打开管理界面", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open_item, &quit_item])?;

    let mut builder = TrayIconBuilder::new()
        .tooltip("Vibe Coding Remote")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => show_main_window(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;
    Ok(())
}

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

async fn restart_server(state: DesktopState) -> Result<(), String> {
    let shutdown = {
        let mut runtime = state.runtime.lock().await;
        runtime.last_error = None;
        runtime.shutdown.take()
    };
    if let Some(shutdown) = shutdown {
        let _ = shutdown.send(());
        let mut stopped = false;
        for _ in 0..40 {
            if !state.runtime.lock().await.running {
                stopped = true;
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        if !stopped {
            return Err("timed out waiting for the previous server instance to stop".to_owned());
        }
    }
    start_server(state).await
}

async fn start_server(state: DesktopState) -> Result<(), String> {
    let config = load_or_create_config(&state.config_path)?;
    let options = RuntimeOptions {
        host: config.host,
        port: config.port,
        auth_token: config.auth_token,
    };
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    {
        let mut runtime = state.runtime.lock().await;
        runtime.running = true;
        runtime.last_error = None;
        runtime.shutdown = Some(shutdown_tx);
    }

    let runtime_state = state.runtime.clone();
    tauri::async_runtime::spawn(async move {
        let result = vibe_coding_remote_server::run_with_shutdown_and_mobile_web(
            options,
            Some(state.mobile_web_dir.clone()),
            async move {
                let _ = shutdown_rx.await;
            },
        )
        .await;

        let mut runtime = runtime_state.lock().await;
        runtime.running = false;
        runtime.shutdown = None;
        if let Err(error) = result {
            runtime.last_error = Some(error.to_string());
        }
    });

    Ok(())
}

fn resolve_mobile_web_dir(app: &tauri::App) -> Result<PathBuf, String> {
    let bundled = app
        .path()
        .resource_dir()
        .map_err(|error| format!("failed to resolve app resource directory: {error}"))?
        .join("mobile-web");
    if bundled.join("index.html").is_file() {
        return Ok(bundled);
    }

    let development = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../web/dist");
    if development.join("index.html").is_file() {
        return Ok(development);
    }

    Err(
        "mobile Web assets are missing; run `pnpm run build:web` before starting the desktop app"
            .to_owned(),
    )
}

fn load_or_create_config(path: &Path) -> Result<DesktopConfig, String> {
    if !path.is_file() {
        let config = DesktopConfig {
            host: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: DEFAULT_PORT,
            auth_token: format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple()),
        };
        write_config(path, &config)?;
        return Ok(config);
    }

    config::Config::builder()
        .add_source(config::File::from(path.to_path_buf()).required(true))
        .build()
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?
        .try_deserialize::<DesktopConfig>()
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))
}

fn write_config(path: &Path, config: &DesktopConfig) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("config path has no parent: {}", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;

    #[cfg(unix)]
    fs::set_permissions(parent, fs::Permissions::from_mode(0o700))
        .map_err(|error| format!("failed to protect {}: {error}", parent.display()))?;

    let body = format!(
        "host = \"{}\"\nport = {}\nauth_token = \"{}\"\n",
        config.host, config.port, config.auth_token
    );

    #[cfg(unix)]
    {
        use std::io::Write;
        let mut file = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .mode(0o600)
            .open(path)
            .map_err(|error| format!("failed to open {}: {error}", path.display()))?;
        file.write_all(body.as_bytes())
            .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
    }

    #[cfg(not(unix))]
    fs::write(path, body)
        .map_err(|error| format!("failed to write {}: {error}", path.display()))?;

    Ok(())
}

fn lan_urls(host: IpAddr, port: u16) -> Vec<String> {
    if host.is_loopback() {
        return Vec::new();
    }

    local_ip_address::list_afinet_netifas()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(_, address)| match address {
            IpAddr::V4(address) if !address.is_loopback() && address.is_private() => {
                Some(format!("http://{address}:{port}"))
            }
            _ => None,
        })
        .collect()
}

fn input_permission_status() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        if vibe_coding_remote_server::desktop_input_permission_is_granted() {
            "granted"
        } else {
            "denied"
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        "not-required"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_config_round_trips_without_losing_the_token() {
        let root = std::env::temp_dir().join(format!("vcr-desktop-test-{}", Uuid::new_v4()));
        let path = root.join("config.toml");
        let first = load_or_create_config(&path).expect("config should be created");
        assert_eq!(first.auth_token.len(), 64);

        let mut updated = first.clone();
        updated.port = 28769;
        write_config(&path, &updated).expect("config should be updated");
        let loaded = load_or_create_config(&path).expect("config should reload");

        assert_eq!(loaded.port, 28769);
        assert_eq!(loaded.auth_token, first.auth_token);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn loopback_configuration_does_not_advertise_lan_urls() {
        assert!(lan_urls(IpAddr::V4(Ipv4Addr::LOCALHOST), 8765).is_empty());
    }
}
