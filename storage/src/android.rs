pub fn write(name: &str, data: &str) -> Option<()> {
	let path = dir()?.join(name);
	match std::fs::write(path, data) {
		Ok(_) => Some(()),
		Err(_) => None,
	}
}

pub fn read(name: &str) -> Option<String> {
	match std::fs::read_to_string(dir()?.join(name)) {
		Ok(s) => Some(s),
		Err(_) => None,
	}
}

/// Application data directory, created if it doesn't exist.
fn dir() -> Option<std::path::PathBuf> {
	let path_string = unsafe {
		let internal_data_path = sapp_android::sapp_internal_data_path();
		let c_str = std::ffi::CStr::from_ptr(internal_data_path);
		c_str.to_string_lossy().into_owned()
	};
	let path = std::path::PathBuf::from(path_string);
	match std::fs::create_dir_all(&path) {
		Ok(_) => Some(path),
		Err(_) => None,
	}
}
