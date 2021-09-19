pub mod deck;
pub mod playing_card;

/// Playing card with value and suit
pub struct PlayingCard {
	value: playing_card::Value,
	suit: playing_card::Suit,
}

/// Deck of playing cards
pub struct Deck {
	/// All cards currently in the deck
	cards: std::vec::Vec<PlayingCard>,
	/// Texture atlas of all cards
	atlas: macroquad::texture::Texture2D,
	/// Scale of loaded atlas texture
	atlas_scale: u16,
}
