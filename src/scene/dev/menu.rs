use macroquad::prelude::*;
use macroquad::ui::hash;

impl super::DevMenu {
	pub async fn new(game: std::rc::Rc<crate::game::Game>) -> Self {
		Self { game }
	}
}

impl crate::scene::Scene for super::DevMenu {
	fn update(&mut self) {
		let window_position = vec2(32_f32, 32_f32);
		let window_size = vec2(screen_width() - 64_f32, screen_height() - 64_f32);

		macroquad::ui::widgets::Window::new(hash!(), window_position, window_size)
			.titlebar(true)
			.movable(false)
			.label("Developer Menu")
			.ui(&mut *macroquad::ui::root_ui(), |ui| {
				let button = macroquad::ui::widgets::Button::new("Deck")
					.position(vec2(8_f32, 8_f32))
					.size(vec2(window_size.x - 16_f32, 96_f32));

				if button.ui(ui) {
					todo!();
				}
			});
	}
}
