use macroquad::prelude::*;

pub mod deck;

/// Scene for testing going through a deck of cards
pub struct DevDeck {
	mini_font: TextParams,
	bold_font: TextParams,
	deck: crate::entity::Deck,
	current_card: crate::entity::PlayingCard,
	skin: macroquad::ui::Skin,
}
