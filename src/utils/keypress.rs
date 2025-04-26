use crate::i18n::t;
use std::io::{self, Read, Write};
use std::process;

/// Waits for user to press any key
///
/// # Arguments
///
/// * `prompt` - Optional message to display. If None, a default message will be shown
/// * `is_en` - Whether to display messages in English
pub fn wait_for_key(prompt: Option<&str>) {
    if let Some(msg) = prompt {
        print!("{} ", msg);
    } else {
        print!("{}", t("wait_for_key"));
    }
    io::stdout().flush().unwrap();
    let mut buffer = [0u8; 1];
    io::stdin().read_exact(&mut buffer).unwrap();
}

/// Waits for user input and then exits the program
///
/// # Arguments
///
/// * `msg` - Optional message to display before exiting. If None, a default message will be shown
pub fn exit_on_keypress(msg: Option<&str>) {
    wait_for_key(msg);
    process::exit(0);
}
