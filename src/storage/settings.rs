use nanoserde::{DeRon, SerRon};

#[derive(DeRon, SerRon)]
pub struct Settings {
	pub account: String,
}
