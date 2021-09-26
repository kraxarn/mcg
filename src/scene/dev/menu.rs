use macroquad::prelude::*;
use macroquad::ui::hash;

impl super::DevMenu {
	pub fn new(assets: std::rc::Rc<crate::assets::Assets>) -> Self {
		Self { assets }
	}
}

impl crate::scene::Scene for super::DevMenu {
	fn update(&mut self) -> crate::scene::Command {
		let window_position = vec2(32_f32, 32_f32);
		let window_size = vec2(screen_width() - 64_f32, screen_height() - 64_f32);

		let mut command = crate::scene::Command::None;

		macroquad::ui::widgets::Window::new(hash!(), window_position, window_size)
			.titlebar(true)
			.movable(false)
			.label("Developer Menu")
			.ui(&mut *macroquad::ui::root_ui(), |ui| {
				let load_deck = macroquad::ui::widgets::Button::new("Deck")
					.position(vec2(8_f32, 8_f32))
					.size(vec2(window_size.x - 16_f32, 96_f32));

				if load_deck.ui(ui) {
					let new_scene = crate::scene::dev::DevDeck::new(self.assets.clone());
					command = crate::scene::Command::PushScene(Box::new(new_scene));
					return;
				}
			});

		command
	}
}
