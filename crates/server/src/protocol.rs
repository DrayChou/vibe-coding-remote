use keyboard_types::Code as KeyboardCode;
use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct ServerActionRequest {
    pub action: ServerAction,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ServerAction {
    InputText { text: String },
    KeySequence { sequence: Vec<KeyChord> },
}

impl ServerAction {
    pub fn validate(self) -> Result<Self, ActionValidationError> {
        match &self {
            Self::InputText { text } if text.is_empty() => Err(ActionValidationError::EmptyText),
            Self::KeySequence { sequence } if sequence.is_empty() => {
                Err(ActionValidationError::EmptySequence)
            }
            Self::KeySequence { sequence }
                if sequence.iter().any(|chord| chord.keys.is_empty()) =>
            {
                Err(ActionValidationError::EmptyChord)
            }
            _ => Ok(self),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct KeyChord {
    #[specta(type = Vec<ServerCode>)]
    pub keys: Vec<KeyboardCode>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct ApiResponse {
    pub ok: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct ServerCapabilitiesResponse {
    #[specta(type = Vec<ServerCode>)]
    pub supported_codes: Vec<KeyboardCode>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct ImportPayload {
    #[serde(rename = "v")]
    pub version: u8,
    pub endpoint: String,
    pub token: String,
}

#[derive(Debug, Clone, Type)]
#[specta(type = String)]
pub struct ServerCode;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, Error)]
pub enum ActionValidationError {
    #[error("text cannot be empty")]
    EmptyText,
    #[error("sequence cannot be empty")]
    EmptySequence,
    #[error("chord keys cannot be empty")]
    EmptyChord,
}
