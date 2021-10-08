use macroquad::prelude::*;

pub mod deck;
pub mod input;
pub mod menu;
pub mod scroll;
pub mod storage;

/// Scene for selecting different kind of dev tests
pub struct DevMenu {
	assets: std::rc::Rc<crate::assets::Assets>,
	title: crate::widget::Title,
	window_size: Vec2,
}

/// Scene for testing going through a deck of cards
pub struct DevDeck {
	bold_font: TextParams,
	deck: crate::entity::Deck,
	current_card: crate::entity::PlayingCard,
	return_texture: Texture2D,
}

/// Scene for testing scrolling through a list
pub struct DevScroll {
	textures: Vec<Texture2D>,
	deck: crate::entity::Deck,
}

/// Scene for testing text input
pub struct DevInput {
	value: String,
}

/// Scene for testing data storage
pub struct DevStorage {
	file_name: String,
	config_dir: String,
	value: String,
}
