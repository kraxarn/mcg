use macroquad::prelude::*;
use macroquad::ui::hash;

impl super::DevInput {
	pub fn new(_assets: std::rc::Rc<crate::assets::Assets>) -> Self {
		Self {
			value: String::new(),
		}
	}
}

impl crate::scene::Scene for super::DevInput {
	fn update(&mut self) -> crate::scene::Command {
		let mut command = crate::scene::Command::None;

		const PADDING: f32 = 16_f32;

		macroquad::ui::widgets::Window::new(
			hash!(),
			vec2(PADDING, screen_height() * 0.25),
			vec2(screen_width() - PADDING * 2_f32, 256_f32),
		)
		.ui(&mut *macroquad::ui::root_ui(), |ui| {
			ui.label(None, "Input text");
			ui.input_text(hash!(), "", &mut self.value);

			if ui.button(None, "Back") {
				command = crate::scene::Command::PopScene;
			}
		});

		command
	}
}
