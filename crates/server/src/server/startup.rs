use crate::{RuntimeOptions, network};

pub(super) fn log_startup_guide(options: &RuntimeOptions) {
    let host = options.host;
    let api_port = options.port;

    tracing::info!("API server");
    let access_status = network::log_access_urls(host, api_port);
    tracing::info!(url = %format!("http://127.0.0.1:{api_port}/health"), "health endpoint");
    tracing::info!(url = %format!("http://127.0.0.1:{api_port}/api/auth-check"), "auth check endpoint");
    tracing::info!(url = %format!("http://127.0.0.1:{api_port}/api/capabilities"), "capabilities endpoint");
    tracing::info!(url = %format!("http://127.0.0.1:{api_port}/api/action"), "action endpoint");
    tracing::info!("Bearer token required for /api/auth-check, /api/capabilities, and /api/action");
    tracing::info!("CORS enabled for cross-origin frontend clients");
    log_import_guide(access_status);
}

fn log_import_guide(access_status: network::ImportEndpointStatus) {
    match access_status {
        network::ImportEndpointStatus::Available { endpoint } => {
            tracing::info!(%endpoint, "mobile API endpoint");
            tracing::info!(
                "open the desktop management window to copy or scan the authenticated phone configuration"
            );
            tracing::info!("the authentication token and import QR are never written to logs");
        }
        network::ImportEndpointStatus::LoopbackOnly => {
            tracing::warn!("mobile import QR unavailable because the server is bound to localhost");
            tracing::warn!("restart with --host 0.0.0.0 to allow phones on the LAN to connect");
        }
        network::ImportEndpointStatus::NoLanAddress => {
            tracing::warn!(
                "mobile import QR unavailable because no private LAN IPv4 address was found"
            );
        }
    }
}
