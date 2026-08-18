use crate::server::{AppState, handlers};
use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::{any, get, post},
};
use std::{path::PathBuf, sync::Arc};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

const MAX_REQUEST_BODY_BYTES: usize = 64 * 1024;

pub(super) fn build_router(auth_token: String, mobile_web_dir: Option<PathBuf>) -> Router {
    let router = Router::new()
        .route("/api/action", post(handlers::execute_action))
        .route("/api/auth-check", get(handlers::auth_check))
        .route("/api/capabilities", get(handlers::capabilities))
        .route("/api/{*path}", any(|| async { StatusCode::NOT_FOUND }))
        .route("/health", get(handlers::health))
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(MAX_REQUEST_BODY_BYTES))
        .with_state(AppState {
            auth_token: Arc::<str>::from(auth_token),
        });

    match mobile_web_dir {
        Some(directory) => router.fallback_service(
            ServeDir::new(&directory)
                .append_index_html_on_directories(true)
                .fallback(ServeFile::new(directory.join("index.html"))),
        ),
        None => router,
    }
}
