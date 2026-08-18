use crate::{input::InputError, protocol::ActionValidationError};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use keyboard_types::Code;

pub(super) struct AppError {
    status: StatusCode,
    message: String,
}

impl AppError {
    pub(super) fn bad_request(error: impl ToString) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: error.to_string(),
        }
    }

    pub(super) fn unauthorized() -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: "missing or invalid authorization token".to_owned(),
        }
    }

    pub(super) fn input_failed(error: InputError) -> Self {
        match error {
            InputError::UnsupportedCode(code) => Self::unsupported_code(code),
            #[cfg(not(any(target_os = "windows", target_os = "macos")))]
            InputError::UnsupportedPlatform => Self {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "desktop input injection is not implemented on this platform yet"
                    .to_owned(),
            },
            other => Self {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: other.to_string(),
            },
        }
    }

    fn unsupported_code(code: Code) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: format!("keyboard code is not supported on this platform yet: {code}"),
        }
    }
}

impl From<ActionValidationError> for AppError {
    fn from(error: ActionValidationError) -> Self {
        Self::bad_request(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}
