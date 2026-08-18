mod cli;
mod export;
mod import_config;
mod input;
mod network;
mod protocol;
mod runtime;
mod server;

pub use export::{ExportError, export_typescript_bindings};
pub use import_config::ImportConfigError;
pub use runtime::{
    RuntimeCommand, RuntimeError, RuntimeOptions, parse_runtime_command, parse_runtime_options,
};
pub use server::ServerError;

pub fn create_import_url(endpoint: String, token: String) -> Result<String, ImportConfigError> {
    import_config::ImportConfig::new(endpoint, token).map(|config| config.import_url)
}

pub async fn run(options: RuntimeOptions) -> Result<(), ServerError> {
    server::run(options).await
}

pub async fn run_with_shutdown(
    options: RuntimeOptions,
    shutdown: impl std::future::Future<Output = ()> + Send + 'static,
) -> Result<(), ServerError> {
    server::run_with_shutdown(options, shutdown).await
}

pub async fn run_with_shutdown_and_mobile_web(
    options: RuntimeOptions,
    mobile_web_dir: Option<std::path::PathBuf>,
    shutdown: impl std::future::Future<Output = ()> + Send + 'static,
) -> Result<(), ServerError> {
    server::run_with_shutdown_and_mobile_web(options, mobile_web_dir, shutdown).await
}

#[cfg(target_os = "macos")]
pub fn desktop_input_permission_is_granted() -> bool {
    input::desktop_input_permission_is_granted()
}

#[cfg(target_os = "macos")]
pub fn request_desktop_input_permission() -> bool {
    input::request_desktop_input_permission()
}
