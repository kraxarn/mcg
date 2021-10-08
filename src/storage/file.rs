/// Write file with specified name
pub fn write(name: &str, data: &str) -> Option<()> {
	let path = dir()?.join(name);

	match std::fs::write(path, data) {
		Ok(_) => Some(()),
		Err(_) => None,
	}
}

/// Read from file with specified name if it exists
pub fn read(name: &str) -> Option<String> {
	match std::fs::read_to_string(dir()?.join(name)) {
		Ok(s) => Some(s),
		Err(_) => None,
	}
}

/// Config directory, created if it doesn't exist.
/// Not guaranteed to make sense, prefer using read/write methods.
pub fn dir() -> Option<std::path::PathBuf> {
	const ORGANIZATION: &str = env!("CARGO_PKG_AUTHORS");
	const APPLICATION: &str = env!("CARGO_PKG_NAME");

	let project_dirs = directories::ProjectDirs::from("com", ORGANIZATION, APPLICATION)?;
	let config_dir = project_dirs.config_dir();

	match std::fs::create_dir_all(config_dir) {
		Ok(_) => Some(std::path::PathBuf::from(config_dir)),
		Err(_) => None,
	}
}
