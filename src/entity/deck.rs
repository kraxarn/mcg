use macroquad::prelude::*;

impl super::Deck {
	/// Get a 52 card deck of playing cards
	pub async fn new() -> Self {
		let scale = Self::texture_scale();

		let mut deck = Self {
			cards: std::vec::Vec::with_capacity(52_usize),
			atlas: Self::load_atlas(scale).await,
			atlas_scale: scale,
		};

		deck.reset();
		deck
	}

	/// Scale needed for textures to not appear blurry at half of window size
	/// (assuming default texture size is 140x190 per card)
	fn texture_scale() -> u16 {
		// Default size for card texture is 140x190
		(screen_width() / 2_f32 / 140_f32).ceil() as u16
	}

	/// Load texture atlas of cards with specific scale
	async fn load_atlas(scale: u16) -> macroquad::texture::Texture2D {
		let mut opt = usvg::Options::default();
		opt.fontdb.load_system_fonts();

		let svg_data = load_file("texture/playing_cards.svg").await.unwrap();
		let tree = usvg::Tree::from_data(&svg_data, &opt.to_ref()).unwrap();

		let pixmap_size = tree.svg_node().size.to_screen_size();
		let width = pixmap_size.width() as u16 * scale;
		let height = pixmap_size.height() as u16 * scale;

		let mut pixmap = tiny_skia::Pixmap::new(width as u32, height as u32).unwrap();
		resvg::render(&tree, usvg::FitTo::Zoom(scale as f32), pixmap.as_mut()).unwrap();

		macroquad::texture::Texture2D::from_rgba8(width, height, pixmap.data())
	}

	// Put all cards back in the deck, in order
	pub fn reset(&mut self) {
		self.cards.clear();

		use strum::IntoEnumIterator;
		for suit in super::playing_card::Suit::iter() {
			for value in super::playing_card::Value::iter() {
				self.cards.push(super::PlayingCard::new(value, suit));
			}
		}
	}

	/// Shuffle deck
	pub fn shuffle(&mut self) {
		fastrand::shuffle(&mut self.cards);
	}

	/// Draw a card, removing it from the deck.
	/// Returns [`None`] if deck is empty
	pub fn draw(&mut self) -> std::option::Option<super::PlayingCard> {
		self.cards.pop()
	}

	/// Get texture atlas of all cards.
	/// Use together with `card_source` to draw a specific card.
	pub fn card_atlas(&self) -> macroquad::texture::Texture2D {
		self.atlas
	}

	/// Size in px of each card.
	pub fn card_size(&self) -> macroquad::math::Vec2 {
		let scale = self.atlas_scale as f32;
		macroquad::math::Vec2::new(140_f32 * scale, 190_f32 * scale)
	}

	/// Get draw params for drawing a specific card, inside, or outside, of the deck
	pub fn card_source(&self, card: &super::PlayingCard) -> macroquad::texture::DrawTextureParams {
		let value = u16::from(card.value as u8);
		let suit = u16::from(card.suit as u8);
		let size = self.card_size();

		let padding = 4_u16 * self.atlas_scale;
		let spacing = 10_u16 * self.atlas_scale;

		let x = padding + (value * size.x as u16) + (spacing * value);
		let y = padding + (suit * size.y as u16) + (spacing * suit);

		DrawTextureParams {
			source: Some(macroquad::math::Rect::new(
				x as f32, y as f32, size.x, size.y,
			)),
			..Default::default()
		}
	}

	/// Number of cards currently in the deck
	pub fn len(&self) -> usize {
		self.cards.len()
	}
}
