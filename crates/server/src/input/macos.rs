use crate::input::{InputError, keymap::MappedKey};
use keyboard_types::Code;
use std::ffi::c_void;

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    fn CGEventCreateKeyboardEvent(
        source: *const c_void,
        virtual_key: u16,
        key_down: bool,
    ) -> *mut c_void;
    fn CGEventPost(tap: u32, event: *const c_void);
    fn CGEventSetFlags(event: *mut c_void, flags: u64);
    fn CGPreflightPostEventAccess() -> bool;
    fn CGRequestPostEventAccess() -> bool;
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFRelease(value: *const c_void);
}

const CG_HID_EVENT_TAP: u32 = 0;
const CG_EVENT_FLAG_MASK_SHIFT: u64 = 0x0002_0000;
const CG_EVENT_FLAG_MASK_CONTROL: u64 = 0x0004_0000;
const CG_EVENT_FLAG_MASK_ALTERNATE: u64 = 0x0008_0000;
const CG_EVENT_FLAG_MASK_COMMAND: u64 = 0x0010_0000;

pub(super) fn send_key_chord(keys: &[MappedKey]) -> Result<(), InputError> {
    if !has_post_event_access() {
        return Err(InputError::InputPermissionDenied);
    }

    let key_codes = keys
        .iter()
        .map(|key| macos_key_code(key.code).ok_or(InputError::UnsupportedCode(key.code)))
        .collect::<Result<Vec<_>, _>>()?;
    let flags = modifier_flags(keys);

    for &key_code in &key_codes {
        post_keyboard_event(key_code, true, flags)?;
    }

    for &key_code in key_codes.iter().rev() {
        post_keyboard_event(key_code, false, flags)?;
    }

    Ok(())
}

pub(super) fn supports_code(code: Code) -> bool {
    macos_key_code(code).is_some()
}

pub(crate) fn input_permission_is_granted() -> bool {
    // SAFETY: The function takes no pointers and only queries the macOS
    // Accessibility permission required for posting synthetic input events.
    unsafe { CGPreflightPostEventAccess() }
}

pub(crate) fn request_input_permission() -> bool {
    if input_permission_is_granted() {
        true
    } else {
        // SAFETY: The function takes no pointers and asks macOS to display the
        // standard Accessibility permission flow for this signed application.
        unsafe { CGRequestPostEventAccess() }
    }
}

fn has_post_event_access() -> bool {
    request_input_permission()
}

fn modifier_flags(keys: &[MappedKey]) -> u64 {
    keys.iter().fold(0, |flags, key| {
        flags
            | match key.code {
                Code::ShiftLeft | Code::ShiftRight => CG_EVENT_FLAG_MASK_SHIFT,
                Code::ControlLeft | Code::ControlRight => CG_EVENT_FLAG_MASK_CONTROL,
                Code::AltLeft | Code::AltRight => CG_EVENT_FLAG_MASK_ALTERNATE,
                Code::MetaLeft | Code::MetaRight => CG_EVENT_FLAG_MASK_COMMAND,
                _ => 0,
            }
    })
}

fn post_keyboard_event(key_code: u16, key_down: bool, flags: u64) -> Result<(), InputError> {
    // SAFETY: A null source is allowed by CoreGraphics. The returned retained
    // CGEvent is posted synchronously and released exactly once below.
    let event = unsafe { CGEventCreateKeyboardEvent(std::ptr::null(), key_code, key_down) };
    if event.is_null() {
        return Err(InputError::CreateKeyboardEventFailed(key_code));
    }

    // SAFETY: `event` is a valid CGEvent returned above and remains alive until
    // after CGEventPost returns. Applying flags directly to each event avoids
    // relying on asynchronous modifier-key state propagation between events.
    unsafe {
        CGEventSetFlags(event, flags);
        CGEventPost(CG_HID_EVENT_TAP, event);
        CFRelease(event);
    }

    Ok(())
}

fn macos_key_code(code: Code) -> Option<u16> {
    Some(match code {
        Code::KeyA => 0,
        Code::KeyS => 1,
        Code::KeyD => 2,
        Code::KeyF => 3,
        Code::KeyH => 4,
        Code::KeyG => 5,
        Code::KeyZ => 6,
        Code::KeyX => 7,
        Code::KeyC => 8,
        Code::KeyV => 9,
        Code::IntlBackslash => 10,
        Code::KeyB => 11,
        Code::KeyQ => 12,
        Code::KeyW => 13,
        Code::KeyE => 14,
        Code::KeyR => 15,
        Code::KeyY => 16,
        Code::KeyT => 17,
        Code::Digit1 => 18,
        Code::Digit2 => 19,
        Code::Digit3 => 20,
        Code::Digit4 => 21,
        Code::Digit6 => 22,
        Code::Digit5 => 23,
        Code::Equal => 24,
        Code::Digit9 => 25,
        Code::Digit7 => 26,
        Code::Minus => 27,
        Code::Digit8 => 28,
        Code::Digit0 => 29,
        Code::BracketRight => 30,
        Code::KeyO => 31,
        Code::KeyU => 32,
        Code::BracketLeft => 33,
        Code::KeyI => 34,
        Code::KeyP => 35,
        Code::Enter => 36,
        Code::KeyL => 37,
        Code::KeyJ => 38,
        Code::Quote => 39,
        Code::KeyK => 40,
        Code::Semicolon => 41,
        Code::Backslash => 42,
        Code::Comma => 43,
        Code::Slash => 44,
        Code::KeyN => 45,
        Code::KeyM => 46,
        Code::Period => 47,
        Code::Tab => 48,
        Code::Space => 49,
        Code::Backquote => 50,
        Code::Backspace => 51,
        Code::Escape => 53,
        Code::MetaRight => 54,
        Code::MetaLeft => 55,
        Code::ShiftLeft => 56,
        Code::CapsLock => 57,
        Code::AltLeft => 58,
        Code::ControlLeft => 59,
        Code::ShiftRight => 60,
        Code::AltRight => 61,
        Code::ControlRight => 62,
        Code::F17 => 64,
        Code::NumpadDecimal => 65,
        Code::NumpadMultiply => 67,
        Code::NumpadAdd => 69,
        Code::NumLock => 71,
        Code::NumpadDivide => 75,
        Code::NumpadEnter => 76,
        Code::NumpadSubtract => 78,
        Code::F18 => 79,
        Code::F19 => 80,
        Code::NumpadEqual => 81,
        Code::Numpad0 => 82,
        Code::Numpad1 => 83,
        Code::Numpad2 => 84,
        Code::Numpad3 => 85,
        Code::Numpad4 => 86,
        Code::Numpad5 => 87,
        Code::Numpad6 => 88,
        Code::Numpad7 => 89,
        Code::F20 => 90,
        Code::Numpad8 => 91,
        Code::Numpad9 => 92,
        Code::F5 => 96,
        Code::F6 => 97,
        Code::F7 => 98,
        Code::F3 => 99,
        Code::F8 => 100,
        Code::F9 => 101,
        Code::F11 => 103,
        Code::F13 => 105,
        Code::F16 => 106,
        Code::F14 => 107,
        Code::F10 => 109,
        Code::F12 => 111,
        Code::F15 => 113,
        Code::Help => 114,
        Code::Home => 115,
        Code::PageUp => 116,
        Code::Delete => 117,
        Code::F4 => 118,
        Code::End => 119,
        Code::F2 => 120,
        Code::PageDown => 121,
        Code::F1 => 122,
        Code::ArrowLeft => 123,
        Code::ArrowRight => 124,
        Code::ArrowDown => 125,
        Code::ArrowUp => 126,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mapped(code: Code) -> MappedKey {
        MappedKey {
            code,
            virtual_key: 0,
            extended: false,
        }
    }

    #[test]
    fn modifier_flags_cover_cross_platform_shortcuts() {
        assert_eq!(
            modifier_flags(&[mapped(Code::MetaLeft), mapped(Code::KeyV)]),
            CG_EVENT_FLAG_MASK_COMMAND
        );
        assert_eq!(
            modifier_flags(&[
                mapped(Code::ControlLeft),
                mapped(Code::AltLeft),
                mapped(Code::ShiftLeft),
                mapped(Code::KeyA),
            ]),
            CG_EVENT_FLAG_MASK_CONTROL | CG_EVENT_FLAG_MASK_ALTERNATE | CG_EVENT_FLAG_MASK_SHIFT
        );
    }
}
