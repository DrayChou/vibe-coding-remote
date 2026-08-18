mod clipboard;
mod executor;
mod keyboard;
mod keymap;
#[cfg(target_os = "macos")]
mod macos;
mod platform;
#[cfg(target_os = "windows")]
mod windows;

use keyboard_types::Code;
use thiserror::Error;

pub use executor::execute_action;
pub(crate) use keymap::supported_codes;

#[cfg(target_os = "macos")]
pub(crate) fn desktop_input_permission_is_granted() -> bool {
    macos::input_permission_is_granted()
}

#[cfg(target_os = "macos")]
pub(crate) fn request_desktop_input_permission() -> bool {
    macos::request_input_permission()
}

#[derive(Debug, Error)]
pub enum InputError {
    #[error("failed to open clipboard: {0}")]
    ClipboardUnavailable(arboard::Error),
    #[error("failed to write text to clipboard: {0}")]
    ClipboardWriteFailed(arboard::Error),
    #[error("keyboard code is not supported on this platform yet: {0}")]
    UnsupportedCode(Code),
    #[cfg(target_os = "windows")]
    #[error("failed to send keyboard input: {0}")]
    SendInputFailed(String),
    #[cfg(target_os = "macos")]
    #[error("macOS Accessibility permission is required to control the focused application")]
    InputPermissionDenied,
    #[cfg(target_os = "macos")]
    #[error("CoreGraphics failed to create a keyboard event for key code {0}")]
    CreateKeyboardEventFailed(u16),
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    #[error("desktop input injection is not implemented on this platform yet")]
    UnsupportedPlatform,
}
