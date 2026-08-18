use crate::{
    input,
    protocol::{ApiResponse, ServerActionRequest, ServerCapabilitiesResponse},
    server::{AppState, auth, error::AppError},
};
use axum::{Json, extract::State, http::HeaderMap};

pub(super) async fn health() -> &'static str {
    "ok"
}

pub(super) async fn auth_check(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse>, AppError> {
    auth::authorize(&headers, state.auth_token.as_ref())?;
    tracing::info!("auth check succeeded");
    Ok(Json(ApiResponse { ok: true }))
}

pub(super) async fn capabilities(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ServerCapabilitiesResponse>, AppError> {
    auth::authorize(&headers, state.auth_token.as_ref())?;

    let supported_codes = input::supported_codes();
    tracing::info!(
        supported_code_count = supported_codes.len(),
        "returned server capabilities"
    );

    Ok(Json(ServerCapabilitiesResponse { supported_codes }))
}

pub(super) async fn execute_action(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<ServerActionRequest>,
) -> Result<Json<ApiResponse>, AppError> {
    auth::authorize(&headers, state.auth_token.as_ref())?;

    let action = payload.action.validate().map_err(|error| {
        tracing::warn!(error = %error, "rejected invalid action");
        AppError::bad_request(error)
    })?;
    let action_kind = match &action {
        crate::protocol::ServerAction::InputText { .. } => "input-text",
        crate::protocol::ServerAction::KeySequence { .. } => "key-sequence",
    };

    input::execute_action(action).map_err(|error| {
        tracing::error!(error = %error, action = action_kind, "failed to execute action");
        AppError::input_failed(error)
    })?;

    tracing::info!(action = action_kind, "executed action");

    Ok(Json(ApiResponse { ok: true }))
}
