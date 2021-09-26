use macroquad::prelude::*;
use macroquad::ui::hash;

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
			mini_font: TextParams {
				font: assets.font(&crate::assets::AssetFont::Mini),
				font_size: 24_u16,
				color: crate::color::FOREGROUND,
				..Default::default()
			},
			bold_font,
			deck,
			current_card,
			skin: Self::skin(bold_font.font_size, assets.clone()),
		}
	}

	fn skin(font_size: u16, assets: std::rc::Rc<crate::assets::Assets>) -> macroquad::ui::Skin {
		let button_style = macroquad::ui::root_ui()
			.style_builder()
			.font(assets.font_data(&crate::assets::AssetFont::Bold))
			.unwrap()
			.font_size(font_size)
			.text_color(crate::color::FOREGROUND)
			.color(crate::color::BUTTON)
			.color_hovered(crate::color::BUTTON_HOVER)
			.color_clicked(crate::color::BUTTON_CLICK)
			.build();

		macroquad::ui::Skin {
			button_style,
			..macroquad::ui::root_ui().default_skin()
		}
	}

	fn draw_button(deck: &mut crate::entity::Deck, card: &mut crate::entity::PlayingCard) {
		let window_size = vec2(screen_width() - 64_f32, 96_f32);
		let window_position = vec2(32_f32, screen_height() - window_size.y - 64_f32);

		macroquad::ui::widgets::Window::new(hash!(), window_position, window_size)
			.titlebar(false)
			.ui(&mut *macroquad::ui::root_ui(), |ui| {
				let button = macroquad::ui::widgets::Button::new(if deck.len() > 0 {
					"Draw card"
				} else {
					"Empty deck"
				})
				.position(glam::Vec2::ZERO)
				.size(window_size);

				if button.ui(ui) {
					match deck.draw() {
						Some(c) => {
							*card = c;
						}
						None => {}
					};
				}
			});
	}
}

impl crate::scene::Scene for super::DevDeck {
	fn update(&mut self) -> crate::scene::Command {
		let card_size = self.deck.card_size();
		let card_width = card_size.x;
		let card_height = card_size.y;

		let window_width = screen_width();
		let window_height = screen_height();

		// Draw FPS
		draw_text_ex(&get_fps().to_string(), 16_f32, 32_f32, self.mini_font);

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

		macroquad::ui::root_ui().push_skin(&self.skin);
		Self::draw_button(&mut self.deck, &mut self.current_card);

		crate::scene::Command::None
	}
}
