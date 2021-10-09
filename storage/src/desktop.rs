/// Write file with specified name
pub fn write(app_name: &str, name: &str, data: &str) -> Option<()> {
	let path = dir(app_name)?.join(name);

	match std::fs::write(path, data) {
		Ok(_) => Some(()),
		Err(_) => None,
	}
}

/// Read from file with specified name if it exists
pub fn read(app_name: &str, name: &str) -> Option<String> {
	match std::fs::read_to_string(dir(app_name)?.join(name)) {
		Ok(s) => Some(s),
		Err(_) => None,
	}
}

/// Config directory, created if it doesn't exist.
fn dir(app_name: &str) -> Option<std::path::PathBuf> {
	let project_dirs = directories::ProjectDirs::from("", "", app_name)?;
	let config_dir = project_dirs.config_dir();

	match std::fs::create_dir_all(config_dir) {
		Ok(_) => Some(std::path::PathBuf::from(config_dir)),
		Err(_) => None,
	}
}
