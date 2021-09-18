pub mod deck;
pub mod playing_card;

/// Playing card with value and suit
pub struct PlayingCard {
	value: playing_card::Value,
	suit: playing_card::Suit,
	texture: std::option::Option<macroquad::texture::Texture2D>,
}

/// Deck of playing cards
pub struct Deck {
	cards: std::vec::Vec<PlayingCard>,
}
