use crate::input::InputError;
use keyboard_types::Code;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MappedKey {
    pub code: Code,
    pub virtual_key: u16,
    pub extended: bool,
}

const SUPPORTED_KEYS: &[MappedKey] = &[
    MappedKey {
        code: Code::Backquote,
        virtual_key: 0xC0,
        extended: false,
    },
    MappedKey {
        code: Code::Backslash,
        virtual_key: 0xDC,
        extended: false,
    },
    MappedKey {
        code: Code::BracketLeft,
        virtual_key: 0xDB,
        extended: false,
    },
    MappedKey {
        code: Code::BracketRight,
        virtual_key: 0xDD,
        extended: false,
    },
    MappedKey {
        code: Code::Comma,
        virtual_key: 0xBC,
        extended: false,
    },
    MappedKey {
        code: Code::Digit0,
        virtual_key: 0x30,
        extended: false,
    },
    MappedKey {
        code: Code::Digit1,
        virtual_key: 0x31,
        extended: false,
    },
    MappedKey {
        code: Code::Digit2,
        virtual_key: 0x32,
        extended: false,
    },
    MappedKey {
        code: Code::Digit3,
        virtual_key: 0x33,
        extended: false,
    },
    MappedKey {
        code: Code::Digit4,
        virtual_key: 0x34,
        extended: false,
    },
    MappedKey {
        code: Code::Digit5,
        virtual_key: 0x35,
        extended: false,
    },
    MappedKey {
        code: Code::Digit6,
        virtual_key: 0x36,
        extended: false,
    },
    MappedKey {
        code: Code::Digit7,
        virtual_key: 0x37,
        extended: false,
    },
    MappedKey {
        code: Code::Digit8,
        virtual_key: 0x38,
        extended: false,
    },
    MappedKey {
        code: Code::Digit9,
        virtual_key: 0x39,
        extended: false,
    },
    MappedKey {
        code: Code::Equal,
        virtual_key: 0xBB,
        extended: false,
    },
    MappedKey {
        code: Code::IntlBackslash,
        virtual_key: 0xE2,
        extended: false,
    },
    MappedKey {
        code: Code::KeyA,
        virtual_key: 0x41,
        extended: false,
    },
    MappedKey {
        code: Code::KeyB,
        virtual_key: 0x42,
        extended: false,
    },
    MappedKey {
        code: Code::KeyC,
        virtual_key: 0x43,
        extended: false,
    },
    MappedKey {
        code: Code::KeyD,
        virtual_key: 0x44,
        extended: false,
    },
    MappedKey {
        code: Code::KeyE,
        virtual_key: 0x45,
        extended: false,
    },
    MappedKey {
        code: Code::KeyF,
        virtual_key: 0x46,
        extended: false,
    },
    MappedKey {
        code: Code::KeyG,
        virtual_key: 0x47,
        extended: false,
    },
    MappedKey {
        code: Code::KeyH,
        virtual_key: 0x48,
        extended: false,
    },
    MappedKey {
        code: Code::KeyI,
        virtual_key: 0x49,
        extended: false,
    },
    MappedKey {
        code: Code::KeyJ,
        virtual_key: 0x4A,
        extended: false,
    },
    MappedKey {
        code: Code::KeyK,
        virtual_key: 0x4B,
        extended: false,
    },
    MappedKey {
        code: Code::KeyL,
        virtual_key: 0x4C,
        extended: false,
    },
    MappedKey {
        code: Code::KeyM,
        virtual_key: 0x4D,
        extended: false,
    },
    MappedKey {
        code: Code::KeyN,
        virtual_key: 0x4E,
        extended: false,
    },
    MappedKey {
        code: Code::KeyO,
        virtual_key: 0x4F,
        extended: false,
    },
    MappedKey {
        code: Code::KeyP,
        virtual_key: 0x50,
        extended: false,
    },
    MappedKey {
        code: Code::KeyQ,
        virtual_key: 0x51,
        extended: false,
    },
    MappedKey {
        code: Code::KeyR,
        virtual_key: 0x52,
        extended: false,
    },
    MappedKey {
        code: Code::KeyS,
        virtual_key: 0x53,
        extended: false,
    },
    MappedKey {
        code: Code::KeyT,
        virtual_key: 0x54,
        extended: false,
    },
    MappedKey {
        code: Code::KeyU,
        virtual_key: 0x55,
        extended: false,
    },
    MappedKey {
        code: Code::KeyV,
        virtual_key: 0x56,
        extended: false,
    },
    MappedKey {
        code: Code::KeyW,
        virtual_key: 0x57,
        extended: false,
    },
    MappedKey {
        code: Code::KeyX,
        virtual_key: 0x58,
        extended: false,
    },
    MappedKey {
        code: Code::KeyY,
        virtual_key: 0x59,
        extended: false,
    },
    MappedKey {
        code: Code::KeyZ,
        virtual_key: 0x5A,
        extended: false,
    },
    MappedKey {
        code: Code::Minus,
        virtual_key: 0xBD,
        extended: false,
    },
    MappedKey {
        code: Code::Period,
        virtual_key: 0xBE,
        extended: false,
    },
    MappedKey {
        code: Code::Quote,
        virtual_key: 0xDE,
        extended: false,
    },
    MappedKey {
        code: Code::Semicolon,
        virtual_key: 0xBA,
        extended: false,
    },
    MappedKey {
        code: Code::Slash,
        virtual_key: 0xBF,
        extended: false,
    },
    MappedKey {
        code: Code::AltLeft,
        virtual_key: 0xA4,
        extended: false,
    },
    MappedKey {
        code: Code::AltRight,
        virtual_key: 0xA5,
        extended: true,
    },
    MappedKey {
        code: Code::Backspace,
        virtual_key: 0x08,
        extended: false,
    },
    MappedKey {
        code: Code::CapsLock,
        virtual_key: 0x14,
        extended: false,
    },
    MappedKey {
        code: Code::ContextMenu,
        virtual_key: 0x5D,
        extended: true,
    },
    MappedKey {
        code: Code::ControlLeft,
        virtual_key: 0xA2,
        extended: false,
    },
    MappedKey {
        code: Code::ControlRight,
        virtual_key: 0xA3,
        extended: true,
    },
    MappedKey {
        code: Code::Enter,
        virtual_key: 0x0D,
        extended: false,
    },
    MappedKey {
        code: Code::MetaLeft,
        virtual_key: 0x5B,
        extended: true,
    },
    MappedKey {
        code: Code::MetaRight,
        virtual_key: 0x5C,
        extended: true,
    },
    MappedKey {
        code: Code::ShiftLeft,
        virtual_key: 0xA0,
        extended: false,
    },
    MappedKey {
        code: Code::ShiftRight,
        virtual_key: 0xA1,
        extended: false,
    },
    MappedKey {
        code: Code::Space,
        virtual_key: 0x20,
        extended: false,
    },
    MappedKey {
        code: Code::Tab,
        virtual_key: 0x09,
        extended: false,
    },
    MappedKey {
        code: Code::Delete,
        virtual_key: 0x2E,
        extended: true,
    },
    MappedKey {
        code: Code::End,
        virtual_key: 0x23,
        extended: true,
    },
    MappedKey {
        code: Code::Help,
        virtual_key: 0x2F,
        extended: false,
    },
    MappedKey {
        code: Code::Home,
        virtual_key: 0x24,
        extended: true,
    },
    MappedKey {
        code: Code::Insert,
        virtual_key: 0x2D,
        extended: true,
    },
    MappedKey {
        code: Code::PageDown,
        virtual_key: 0x22,
        extended: true,
    },
    MappedKey {
        code: Code::PageUp,
        virtual_key: 0x21,
        extended: true,
    },
    MappedKey {
        code: Code::ArrowDown,
        virtual_key: 0x28,
        extended: true,
    },
    MappedKey {
        code: Code::ArrowLeft,
        virtual_key: 0x25,
        extended: true,
    },
    MappedKey {
        code: Code::ArrowRight,
        virtual_key: 0x27,
        extended: true,
    },
    MappedKey {
        code: Code::ArrowUp,
        virtual_key: 0x26,
        extended: true,
    },
    MappedKey {
        code: Code::NumLock,
        virtual_key: 0x90,
        extended: false,
    },
    MappedKey {
        code: Code::Numpad0,
        virtual_key: 0x60,
        extended: false,
    },
    MappedKey {
        code: Code::Numpad1,
        virtual_key: 0x61,
        extended: false,
    },
    MappedKey {
        code: Code::Numpad2,
        virtual_key: 0x62,
        extended: false,
    },
    MappedKey {
        code: Code::Numpad3,
        virtual_key: 0x63,
        extended: false,
    },
    MappedKey {
        code: Code::Numpad4,
        virtual_key: 0x64,
        extended: false,
    },
    MappedKey {
        code: Code::Numpad5,
        virtual_key: 0x65,
        extended: false,
    },
    MappedKey {
        code: Code::Numpad6,
        virtual_key: 0x66,
        extended: false,
    },
    MappedKey {
        code: Code::Numpad7,
        virtual_key: 0x67,
        extended: false,
    },
    MappedKey {
        code: Code::Numpad8,
        virtual_key: 0x68,
        extended: false,
    },
    MappedKey {
        code: Code::Numpad9,
        virtual_key: 0x69,
        extended: false,
    },
    MappedKey {
        code: Code::NumpadAdd,
        virtual_key: 0x6B,
        extended: false,
    },
    MappedKey {
        code: Code::NumpadDecimal,
        virtual_key: 0x6E,
        extended: false,
    },
    MappedKey {
        code: Code::NumpadDivide,
        virtual_key: 0x6F,
        extended: true,
    },
    MappedKey {
        code: Code::NumpadEnter,
        virtual_key: 0x0D,
        extended: true,
    },
    MappedKey {
        code: Code::NumpadMultiply,
        virtual_key: 0x6A,
        extended: false,
    },
    MappedKey {
        code: Code::NumpadSubtract,
        virtual_key: 0x6D,
        extended: false,
    },
    MappedKey {
        code: Code::Escape,
        virtual_key: 0x1B,
        extended: false,
    },
    MappedKey {
        code: Code::PrintScreen,
        virtual_key: 0x2C,
        extended: true,
    },
    MappedKey {
        code: Code::ScrollLock,
        virtual_key: 0x91,
        extended: false,
    },
    MappedKey {
        code: Code::Pause,
        virtual_key: 0x13,
        extended: false,
    },
    MappedKey {
        code: Code::BrowserBack,
        virtual_key: 0xA6,
        extended: true,
    },
    MappedKey {
        code: Code::BrowserFavorites,
        virtual_key: 0xAB,
        extended: true,
    },
    MappedKey {
        code: Code::BrowserForward,
        virtual_key: 0xA7,
        extended: true,
    },
    MappedKey {
        code: Code::BrowserHome,
        virtual_key: 0xAC,
        extended: true,
    },
    MappedKey {
        code: Code::BrowserRefresh,
        virtual_key: 0xA8,
        extended: true,
    },
    MappedKey {
        code: Code::BrowserSearch,
        virtual_key: 0xAA,
        extended: true,
    },
    MappedKey {
        code: Code::BrowserStop,
        virtual_key: 0xA9,
        extended: true,
    },
    MappedKey {
        code: Code::LaunchApp1,
        virtual_key: 0xB6,
        extended: true,
    },
    MappedKey {
        code: Code::LaunchApp2,
        virtual_key: 0xB7,
        extended: true,
    },
    MappedKey {
        code: Code::LaunchMail,
        virtual_key: 0xB4,
        extended: true,
    },
    MappedKey {
        code: Code::MediaPlayPause,
        virtual_key: 0xB3,
        extended: true,
    },
    MappedKey {
        code: Code::MediaSelect,
        virtual_key: 0xB5,
        extended: true,
    },
    MappedKey {
        code: Code::MediaStop,
        virtual_key: 0xB2,
        extended: true,
    },
    MappedKey {
        code: Code::MediaTrackNext,
        virtual_key: 0xB0,
        extended: true,
    },
    MappedKey {
        code: Code::MediaTrackPrevious,
        virtual_key: 0xB1,
        extended: true,
    },
    MappedKey {
        code: Code::Sleep,
        virtual_key: 0x5F,
        extended: false,
    },
    MappedKey {
        code: Code::AudioVolumeDown,
        virtual_key: 0xAE,
        extended: true,
    },
    MappedKey {
        code: Code::AudioVolumeMute,
        virtual_key: 0xAD,
        extended: true,
    },
    MappedKey {
        code: Code::AudioVolumeUp,
        virtual_key: 0xAF,
        extended: true,
    },
    MappedKey {
        code: Code::F1,
        virtual_key: 0x70,
        extended: false,
    },
    MappedKey {
        code: Code::F2,
        virtual_key: 0x71,
        extended: false,
    },
    MappedKey {
        code: Code::F3,
        virtual_key: 0x72,
        extended: false,
    },
    MappedKey {
        code: Code::F4,
        virtual_key: 0x73,
        extended: false,
    },
    MappedKey {
        code: Code::F5,
        virtual_key: 0x74,
        extended: false,
    },
    MappedKey {
        code: Code::F6,
        virtual_key: 0x75,
        extended: false,
    },
    MappedKey {
        code: Code::F7,
        virtual_key: 0x76,
        extended: false,
    },
    MappedKey {
        code: Code::F8,
        virtual_key: 0x77,
        extended: false,
    },
    MappedKey {
        code: Code::F9,
        virtual_key: 0x78,
        extended: false,
    },
    MappedKey {
        code: Code::F10,
        virtual_key: 0x79,
        extended: false,
    },
    MappedKey {
        code: Code::F11,
        virtual_key: 0x7A,
        extended: false,
    },
    MappedKey {
        code: Code::F12,
        virtual_key: 0x7B,
        extended: false,
    },
    MappedKey {
        code: Code::F13,
        virtual_key: 0x7C,
        extended: false,
    },
    MappedKey {
        code: Code::F14,
        virtual_key: 0x7D,
        extended: false,
    },
    MappedKey {
        code: Code::F15,
        virtual_key: 0x7E,
        extended: false,
    },
    MappedKey {
        code: Code::F16,
        virtual_key: 0x7F,
        extended: false,
    },
    MappedKey {
        code: Code::F17,
        virtual_key: 0x80,
        extended: false,
    },
    MappedKey {
        code: Code::F18,
        virtual_key: 0x81,
        extended: false,
    },
    MappedKey {
        code: Code::F19,
        virtual_key: 0x82,
        extended: false,
    },
    MappedKey {
        code: Code::F20,
        virtual_key: 0x83,
        extended: false,
    },
    MappedKey {
        code: Code::F21,
        virtual_key: 0x84,
        extended: false,
    },
    MappedKey {
        code: Code::F22,
        virtual_key: 0x85,
        extended: false,
    },
    MappedKey {
        code: Code::F23,
        virtual_key: 0x86,
        extended: false,
    },
    MappedKey {
        code: Code::F24,
        virtual_key: 0x87,
        extended: false,
    },
];

pub(super) fn resolve_key(code: Code) -> Result<MappedKey, InputError> {
    SUPPORTED_KEYS
        .iter()
        .find(|mapped| mapped.code == code)
        .copied()
        .ok_or(InputError::UnsupportedCode(code))
}

pub(crate) fn supported_codes() -> Vec<Code> {
    SUPPORTED_KEYS
        .iter()
        .map(|mapped| mapped.code)
        .filter(|code| crate::input::platform::supports_code(*code))
        .collect()
}
