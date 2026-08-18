use crate::input::{InputError, keyboard, platform};
use std::{thread, time::Duration};

const CLIPBOARD_SETTLE_DELAY_MS: u64 = 20;

pub(super) fn input_text(text: &str) -> Result<(), InputError> {
    let mut clipboard = arboard::Clipboard::new().map_err(InputError::ClipboardUnavailable)?;
    clipboard
        .set_text(text)
        .map_err(InputError::ClipboardWriteFailed)?;

    thread::sleep(Duration::from_millis(CLIPBOARD_SETTLE_DELAY_MS));
    keyboard::send_key_chord(platform::paste_chord())
}
