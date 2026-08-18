use crate::input::{InputError, keymap::MappedKey};
use keyboard_types::Code;

#[cfg(target_os = "macos")]
use crate::input::macos;
#[cfg(target_os = "windows")]
use crate::input::windows;

pub(super) fn send_key_chord(keys: &[MappedKey]) -> Result<(), InputError> {
    #[cfg(target_os = "windows")]
    {
        windows::send_key_chord(keys)
    }

    #[cfg(target_os = "macos")]
    {
        macos::send_key_chord(keys)
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let _ = keys;
        Err(InputError::UnsupportedPlatform)
    }
}

pub(super) fn supports_code(code: Code) -> bool {
    #[cfg(target_os = "macos")]
    {
        macos::supports_code(code)
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = code;
        true
    }
}

pub(super) fn paste_chord() -> &'static [Code] {
    #[cfg(target_os = "macos")]
    {
        &[Code::MetaLeft, Code::KeyV]
    }

    #[cfg(not(target_os = "macos"))]
    {
        &[Code::ControlLeft, Code::KeyV]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paste_uses_the_native_platform_modifier() {
        #[cfg(target_os = "macos")]
        assert_eq!(paste_chord(), &[Code::MetaLeft, Code::KeyV]);

        #[cfg(target_os = "windows")]
        assert_eq!(paste_chord(), &[Code::ControlLeft, Code::KeyV]);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn macos_exposes_keys_used_by_the_mobile_action_panel() {
        for code in [
            Code::Enter,
            Code::Escape,
            Code::Tab,
            Code::Backspace,
            Code::ArrowUp,
            Code::ArrowDown,
            Code::ArrowLeft,
            Code::ArrowRight,
            Code::ControlLeft,
            Code::ShiftLeft,
            Code::MetaLeft,
            Code::KeyC,
            Code::KeyV,
        ] {
            assert!(supports_code(code), "missing macOS key mapping for {code}");
        }
    }
}
