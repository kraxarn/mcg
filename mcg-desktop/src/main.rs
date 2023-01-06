use std::panic;
use native_dialog::{MessageDialog, MessageType};

fn main() {
	// TODO: Show in main window
	panic::set_hook(Box::new(move |info| {
		let mut message = if let Some(payload) = info.payload().downcast_ref::<&str>() {
			String::from(*payload)
		} else {
			String::from("panic")
		};

		if let Some(location) = info.location() {
			message.push_str(&format!("\nin {}:{}", location.file(), location.line()));
		}

		MessageDialog::new()
			.set_type(MessageType::Error)
			.set_title(&format!("{} crashed!", env!("CARGO_PKG_NAME")))
			.set_text(&message)
			.show_alert()
			.unwrap();
	}));

	mcg::run();
}