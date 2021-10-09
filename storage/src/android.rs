/// Write file to internal data path with specified name
pub fn write(name: &str, data: &str) -> std::io::Result<()> {
	let path = dir()?.join(name);
	std::fs::write(path, data)
}

/// Read file from internal data path with specified name if it exists
pub fn read(name: &str) -> std::io::Result<String> {
	std::fs::read_to_string(dir()?.join(name))
}

/// Application data directory, created if it doesn't exist.
fn dir() -> std::io::Result<std::path::PathBuf> {
	let path_string = unsafe {
		let internal_data_path = sapp_android::sapp_internal_data_path();
		let c_str = std::ffi::CStr::from_ptr(internal_data_path);
		c_str.to_string_lossy().into_owned()
	};
	let path = std::path::PathBuf::from(path_string);

	match std::fs::create_dir_all(&path) {
		Ok(_) => Ok(path),
		Err(e) => Err(e),
	}
}
