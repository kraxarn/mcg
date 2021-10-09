#[cfg(any(target_os = "linux", target_os = "macos", windows))]
mod desktop;

#[cfg(target_os = "android")]
mod android;

#[cfg(target_arch = "wasm32")]
mod wasm;

/// Write file
pub fn write(app_name: &str, name: &str, data: &str) -> Option<()> {
	#[cfg(any(target_os = "linux", target_os = "macos", windows))]
	{
		return desktop::write(app_name, name, data);
	}
	#[cfg(target_os = "android")]
	{
		return android::write(name, data);
	}
	#[cfg(target_arch = "wasm32")]
	{
		return wasm::write(name, data);
	}
	None
}

/// Read file
pub fn read(app_name: &str, name: &str) -> Option<String> {
	#[cfg(any(target_os = "linux", target_os = "macos", windows))]
	{
		return desktop::read(app_name, name);
	}
	#[cfg(target_os = "android")]
	{
		return android::read(name);
	}
	#[cfg(target_arch = "wasm32")]
	{
		return wasm::read(name);
	}
	None
}
