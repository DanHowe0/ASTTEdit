use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

fn show_message(message: &str, level: MessageLevel) {
    MessageDialog::new()
        .set_title("ASTTE")
        .set_description(message)
        .set_level(level)
        .set_buttons(MessageButtons::Ok)
        .show();
}

pub fn show_error(message: &str) {
    show_message(message, MessageLevel::Error);
}

pub fn show_update_available(version: &str, url: &str) {
    let result = MessageDialog::new()
        .set_title("ASTTE update available")
        .set_description(&format!("ASTTE {version} is available.\n\nOpen the download page?\n{url}"))
        .set_level(MessageLevel::Info)
        .set_buttons(MessageButtons::YesNo)
        .show();

    if result == MessageDialogResult::Yes {
        let _ = webbrowser::open(url);
    }
}