pub mod playing_card;

pub struct PlayingCard {
	value: playing_card::Value,
	suit: playing_card::Suit,
	texture: std::option::Option<macroquad::texture::Texture2D>,
}
