use std::fmt::{Display, Formatter};
use bevy::prelude::Component;
use crate::enums::{PlayingCardSuit, PlayingCardValue};
use crate::textures::PlayingCardTexture;

/// Playing card with value and suit
#[derive(Component, Copy, Clone)]
pub struct PlayingCard {
	pub(crate) value: PlayingCardValue,
	pub(crate) suit: PlayingCardSuit,
}

impl PlayingCard {
	pub fn new(value: PlayingCardValue, suit: PlayingCardSuit) -> Self {
		Self { value, suit }
	}

	pub fn sprite_index(&self) -> usize {
		let row = self.suit as usize;
		let column = self.value as usize;
		column + (row * PlayingCardTexture::atlas_columns())
	}
}

impl Display for PlayingCard {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} of {}", self.value, self.suit)
	}
}