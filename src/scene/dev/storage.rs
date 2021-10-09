use macroquad::prelude::*;
use macroquad::ui::hash;

impl super::DevStorage {
	pub fn new(_assets: std::rc::Rc<crate::assets::Assets>) -> Self {
		Self {
			settings: crate::settings::Settings::read(),
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
				ui.label(None, &format!("count: {}", self.settings.count));

				if ui.button(None, "Read") {
					self.settings = crate::settings::Settings::read();
				}

				if ui.button(None, "Write") {
					self.settings.write();
				}

				if ui.button(None, "Add") {
					self.settings.count += 1;
				}

				if ui.button(None, "Back") {
					command = crate::scene::Command::PopScene;
				}
			});

		command
	}
}
