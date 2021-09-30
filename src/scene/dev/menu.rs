use macroquad::prelude::*;
use macroquad::ui::hash;

impl super::DevMenu {
	pub fn new(assets: std::rc::Rc<crate::assets::Assets>) -> Self {
		Self { assets }
	}
}

impl crate::scene::Scene for super::DevMenu {
	fn update(&mut self) -> crate::scene::Command {
		const WINDOW_PADDING: f32 = 16_f32;
		const BUTTON_PADDING: f32 = 24_f32;

		let window_position = vec2(WINDOW_PADDING, WINDOW_PADDING);
		let window_size = vec2(
			screen_width() - WINDOW_PADDING * 2_f32,
			screen_height() - WINDOW_PADDING * 2_f32,
		);

		let mut command = crate::scene::Command::None;

		macroquad::ui::widgets::Window::new(hash!(), window_position, window_size)
			.titlebar(true)
			.movable(false)
			.label("Developer Menu")
			.ui(&mut *macroquad::ui::root_ui(), |ui| {
				let load_deck = macroquad::ui::widgets::Button::new("Deck")
					.position(vec2(BUTTON_PADDING, BUTTON_PADDING / 2_f32))
					.size(vec2(window_size.x - BUTTON_PADDING * 2_f32, 96_f32));

				if load_deck.ui(ui) {
					let new_scene = crate::scene::dev::DevDeck::new(self.assets.clone());
					command = crate::scene::Command::PushScene(Box::new(new_scene));
					return;
				}
			});

		command
	}
}
