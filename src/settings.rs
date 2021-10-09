use nanoserde::{DeRon, SerRon};

const FILE_NAME: &str = "settings.ron";

/// Application settings
#[derive(DeRon, SerRon)]
pub struct Settings {
	/// Logged in account, may change type
	pub account: Option<String>,
	/// Counter for testing
	pub count: u32,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			account: None,
			count: 0_u32,
		}
	}
}

impl Settings {
	/// Load settings from storage, or get default settings
	pub fn read() -> Self {
		match mcg_storage::read(crate::APP_NAME, FILE_NAME) {
			Err(_) => Default::default(),
			Ok(s) => match nanoserde::DeRon::deserialize_ron(&s) {
				Ok(s) => s,
				Err(_) => Default::default(),
			},
		}
	}

	/// Save settings to storage
	pub fn write(&self) -> bool {
		let data = nanoserde::SerRon::serialize_ron(self);
		match mcg_storage::write(crate::APP_NAME, FILE_NAME, &data) {
			Err(_) => false,
			Ok(_) => true,
		}
	}
}
