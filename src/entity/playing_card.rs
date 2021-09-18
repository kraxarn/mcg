use macroquad::prelude::*;

#[derive(Copy, Clone, strum_macros::Display, strum_macros::EnumIter)]
pub enum Value {
	#[strum(serialize = "ace")]
	Ace = 1,
	#[strum(serialize = "2")]
	Two = 2,
	#[strum(serialize = "3")]
	Three = 3,
	#[strum(serialize = "4")]
	Four = 4,
	#[strum(serialize = "5")]
	Five = 5,
	#[strum(serialize = "6")]
	Six = 6,
	#[strum(serialize = "7")]
	Seven = 7,
	#[strum(serialize = "8")]
	Eight = 8,
	#[strum(serialize = "9")]
	Nine = 9,
	#[strum(serialize = "10")]
	Ten = 10,
	#[strum(serialize = "jack")]
	Jack = 11,
	#[strum(serialize = "queen")]
	Queen = 12,
	#[strum(serialize = "king")]
	King = 13,
}

#[derive(Copy, Clone, strum_macros::Display, strum_macros::EnumIter)]
pub enum Suit {
	#[strum(serialize = "clubs")]
	Clubs,
	#[strum(serialize = "diamonds")]
	Diamonds,
	#[strum(serialize = "hearts")]
	Hearts,
	#[strum(serialize = "spades")]
	Spades,
}

impl super::PlayingCard {
	fn new(value: Value, suit: Suit) -> Self {
		Self {
			value,
			suit,
			texture: None,
		}
	}

	/// Path to texture relative to assets directory
	pub fn asset_path(&self) -> String {
		format!("texture/card/{}/{}.png", self.suit, self.value as i32)
	}

	/// Loads the card texture, if not already loaded.
	/// Panics on failure, maybe it should return a Result instead?
	pub async fn load_texture(&mut self) {
		if self.texture.is_some() {
			return;
		}
		self.texture = Some(
			macroquad::texture::load_texture(&self.asset_path())
				.await
				.unwrap(),
		);
	}

	/// Returns card texture, panics if not already loaded with load_texture
	pub fn texture(&mut self) -> macroquad::texture::Texture2D {
		match self.texture {
			Some(t) => t,
			None => {
				panic!("texture not loaded, use load_texture before drawing");
			}
		}
	}
}

impl std::fmt::Display for super::PlayingCard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} of {}", self.value, self.suit)
	}
}

/// Get an entire 52 card deck of playing cards
pub fn deck() -> std::vec::Vec<super::PlayingCard> {
	let mut cards: std::vec::Vec<super::PlayingCard> = std::vec::Vec::with_capacity(52_usize);

	use strum::IntoEnumIterator;
	for suit in Suit::iter() {
		for value in Value::iter() {
			cards.push(super::PlayingCard::new(value, suit));
		}
	}

	cards
}

/// Shuffle deck of cards
pub fn shuffle(cards: &mut std::vec::Vec<super::PlayingCard>) {
	fastrand::shuffle(cards);
}
