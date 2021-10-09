/// Write file with specified name
pub fn write(app_name: &str, name: &str, data: &str) -> std::io::Result<()> {
	let path = dir(app_name)?.join(name);
	std::fs::write(path, data)
}

/// Read from file with specified name if it exists
pub fn read(app_name: &str, name: &str) -> std::io::Result<String> {
	std::fs::read_to_string(dir(app_name)?.join(name))
}

/// Config directory, created if it doesn't exist.
fn dir(app_name: &str) -> std::io::Result<std::path::PathBuf> {
	let project_dirs = match directories::ProjectDirs::from("", "", app_name) {
		None => Err(std::io::Error::new(
			std::io::ErrorKind::NotFound,
			"No such directory",
		)),
		Some(d) => Ok(d),
	}?;
	let config_dir = project_dirs.config_dir();

	match std::fs::create_dir_all(config_dir) {
		Ok(_) => Ok(std::path::PathBuf::from(config_dir)),
		Err(e) => Err(e),
	}
}
