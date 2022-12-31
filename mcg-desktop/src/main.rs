use std::panic;
use native_dialog::{MessageDialog, MessageType};

fn main() {
	// TODO: Show in main window
	panic::set_hook(Box::new(move |info| {
		MessageDialog::new()
			.set_type(MessageType::Error)
			.set_title(&format!("{} crashed!", env!("CARGO_PKG_NAME")))
			.set_text(&format!("{:?}", info))
			.show_alert()
			.unwrap();
	}));
	mcg::run();
}