use macroquad::prelude::*;

pub mod deck;
pub mod menu;

/// Scene for selecting different kind of dev tests
pub struct DevMenu {
	assets: std::rc::Rc<crate::assets::Assets>,
	title: crate::widget::Title,
}

/// Scene for testing going through a deck of cards
pub struct DevDeck {
	mini_font: TextParams,
	bold_font: TextParams,
	deck: crate::entity::Deck,
	current_card: crate::entity::PlayingCard,
	return_texture: Texture2D,
}