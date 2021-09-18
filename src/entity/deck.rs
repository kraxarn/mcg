use macroquad::prelude::{load_texture, DrawTextureParams};

impl super::Deck {
	/// Get a 52 card deck of playing cards
	pub async fn new() -> Self {
		let mut deck = Self {
			cards: std::vec::Vec::with_capacity(52_usize),
			atlas: load_texture("texture/playing_cards.png").await.unwrap(),
		};

		deck.reset();
		deck
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
	/// Currently always 140x190px.
	pub fn card_size(&self) -> macroquad::math::Vec2 {
		macroquad::math::Vec2::new(140_f32, 190_f32)
	}

	/// Get draw params for drawing a specific card, inside, or outside, of the deck
	pub fn card_source(&self, card: &super::PlayingCard) -> macroquad::texture::DrawTextureParams {
		// Texture has a 4px padding
		// All cards have 10px gutters

		let value = f32::from(card.value as u8);
		let suit = f32::from(card.suit as u8);
		let size = self.card_size();

		let x = 4_f32 + (value * size.x) + (10_f32 * value);
		let y = 4_f32 + (suit * size.y) + (10_f32 * suit);

		DrawTextureParams {
			dest_size: Some(self.card_size()),
			source: Some(macroquad::math::Rect::new(x, y, size.x, size.y)),
			..Default::default()
		}
	}
}
