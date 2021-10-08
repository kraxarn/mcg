use macroquad::prelude::*;
use macroquad::ui::hash;

impl super::DevStorage {
	pub fn new(_assets: std::rc::Rc<crate::assets::Assets>) -> Self {
		let file_name = format!("debug_file.dat");

		Self {
			config_dir: match crate::storage::file::dir() {
				Some(p) => format!("{}", p.join(&file_name).display()),
				None => "Invalid path".to_owned(),
			},
			file_name,
			value: "(none)".to_owned(),
		}
	}
}

impl crate::scene::Scene for super::DevStorage {
	fn update(&mut self) -> crate::scene::Command {
		let window_position = vec2(crate::style::WINDOW_PADDING, screen_height() / 3_f32);
		let window_size = vec2(
			screen_width() - crate::style::WINDOW_PADDING * 2_f32,
			screen_height() - window_position.y - crate::style::WINDOW_PADDING * 2_f32,
		);

		let mut command = crate::scene::Command::None;

		macroquad::ui::widgets::Window::new(hash!(), window_position, window_size)
			.titlebar(false)
			.movable(false)
			.ui(&mut *macroquad::ui::root_ui(), |ui| {
				ui.label(None, &self.config_dir);
				ui.label(None, &self.file_name);
				ui.label(None, &self.value);

				if ui.button(None, "Read") {
					self.value = match crate::storage::file::read(&self.file_name) {
						None => "(empty)".to_owned(),
						Some(s) => s,
					}
				}

				if ui.button(None, "Write") {
					self.value = match crate::storage::file::write(&self.file_name, &self.value) {
						None => "(write failed)".to_owned(),
						Some(_) => format!("{} (saved)", self.value),
					}
				}

				if ui.button(None, "Add") {
					self.value = match self.value.parse::<u32>() {
						Ok(v) => (v + 1).to_string(),
						Err(_) => "0".to_owned(),
					}
				}

				if ui.button(None, "Back") {
					command = crate::scene::Command::PopScene;
				}
			});

		command
	}
}
