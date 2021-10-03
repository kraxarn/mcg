use macroquad::prelude::*;

impl super::DevDeck {
	pub fn new(assets: std::rc::Rc<crate::assets::Assets>) -> Self {
		let bold_font = TextParams {
			font: assets.font(&crate::assets::AssetFont::Bold),
			font_size: 28_u16,
			color: crate::color::FOREGROUND,
			..Default::default()
		};

		let mut deck = crate::entity::Deck::new(assets.clone());
		deck.shuffle();
		let current_card = deck.draw().unwrap();

		Self {
			bold_font,
			deck,
			current_card,
			return_texture: assets.tileset_texture(
				&crate::assets::AssetTexture::Ui,
				crate::tileset::Ui::Return as u16,
			),
		}
	}

	fn draw_ui(&mut self) -> crate::scene::Command {
		// Draw card button
		let button_text = if self.deck.len() > 0 {
			format!("Draw card {}", 53 - self.deck.len())
		} else {
			"Empty deck".to_owned()
		};
		let button_size = vec2(screen_width() - 64_f32, 96_f32);
		let button_position = vec2(32_f32, screen_height() - button_size.y - 64_f32);

		let card_button = macroquad::ui::widgets::Button::new(button_text)
			.position(button_position)
			.size(button_size);

		// Go back button
		let return_button = macroquad::ui::widgets::Button::new(self.return_texture)
			.position(vec2(32_f32, 32_f32))
			.size(vec2(96_f32, 96_f32));

		if card_button.ui(&mut *macroquad::ui::root_ui()) {
			match self.deck.draw() {
				Some(c) => {
					self.current_card = c;
				}
				None => {}
			};
		}

		if return_button.ui(&mut *macroquad::ui::root_ui()) {
			return crate::scene::Command::PopScene;
		}

		crate::scene::Command::None
	}
}

impl crate::scene::Scene for super::DevDeck {
	fn update(&mut self) -> crate::scene::Command {
		let card_size = self.deck.card_size();
		let card_width = card_size.x;
		let card_height = card_size.y;

		let window_width = screen_width();
		let window_height = screen_height();

		// Card texture scale (40% of screen width)
		let card_scale = (window_width * 0.4) / card_width;

		// Draw the card itself
		let card_y = (window_height / 2_f32) - (card_height * card_scale / 2_f32);
		draw_texture_ex(
			self.deck.card_atlas(),
			(window_width / 2_f32) - (card_width * card_scale / 2_f32),
			card_y,
			WHITE,
			DrawTextureParams {
				dest_size: Some(card_size * card_scale),
				..self.deck.card_source(&self.current_card)
			},
		);

		// Measure card name size to center text
		let card_name = self.current_card.to_string();
		let card_name_width = measure_text(
			&card_name,
			Some(self.bold_font.font),
			self.bold_font.font_size as u16,
			self.bold_font.font_scale,
		)
		.width;

		// Draw name of card centered under card
		draw_text_ex(
			&card_name,
			(window_width / 2_f32) - (card_name_width / 2_f32),
			card_y + card_height * card_scale + 64_f32,
			self.bold_font,
		);

		self.draw_ui()
	}
}
