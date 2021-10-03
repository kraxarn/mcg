use macroquad::prelude::*;
use macroquad::ui::hash;

impl super::DevScroll {
	pub fn new(assets: std::rc::Rc<crate::assets::Assets>) -> Self {
		let mut deck = crate::entity::Deck::new(assets.clone());

		let mut textures = Vec::new();
		while let Some(c) = deck.draw() {
			textures.push(deck.card_texture(&c));
		}

		Self { textures, deck }
	}
}

impl crate::scene::Scene for super::DevScroll {
	fn update(&mut self) -> crate::scene::Command {
		let window_position = vec2(crate::style::WINDOW_PADDING, crate::style::WINDOW_PADDING);
		let window_size = vec2(
			screen_width() - crate::style::WINDOW_PADDING * 2_f32,
			screen_height() - crate::style::WINDOW_PADDING * 2_f32,
		);

		let group_size = self.deck.card_size() / 2_f32;
		let card_size = group_size * 0.98_f32;

		macroquad::ui::widgets::Window::new(hash!(), window_position, window_size)
			.titlebar(false)
			.movable(false)
			.ui(&mut *macroquad::ui::root_ui(), |ui| {
				for i in 0..self.textures.len() {
					ui.group(hash!(i), group_size, |ui| {
						ui.texture(
							self.textures.get(i).unwrap().clone(),
							card_size.x,
							card_size.y,
						);
					});
				}
			});

		crate::scene::Command::None
	}
}
