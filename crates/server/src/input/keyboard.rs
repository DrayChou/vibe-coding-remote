use crate::{
    input::{
        InputError,
        keymap::{self, MappedKey},
        platform,
    },
    protocol::KeyChord,
};
use keyboard_types::Code;

pub(super) fn send_key_sequence(sequence: &[KeyChord]) -> Result<(), InputError> {
    for chord in sequence {
        send_key_chord(&chord.keys)?;
    }

    Ok(())
}

pub(super) fn send_key_chord(keys: &[Code]) -> Result<(), InputError> {
    let resolved = keys
        .iter()
        .copied()
        .map(keymap::resolve_key)
        .collect::<Result<Vec<_>, _>>()?;
    let ordered = order_chord_keys(&resolved);
    platform::send_key_chord(&ordered)
}

fn order_chord_keys(keys: &[MappedKey]) -> Vec<MappedKey> {
    let mut ordered = Vec::with_capacity(keys.len());

    for &key in keys {
        if is_modifier(key.code) {
            ordered.push(key);
        }
    }

    for &key in keys {
        if !is_modifier(key.code) {
            ordered.push(key);
        }
    }

    ordered
}

fn is_modifier(code: Code) -> bool {
    matches!(
        code,
        Code::AltLeft
            | Code::AltRight
            | Code::ControlLeft
            | Code::ControlRight
            | Code::MetaLeft
            | Code::MetaRight
            | Code::ShiftLeft
            | Code::ShiftRight
    )
}
