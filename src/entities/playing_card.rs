use std::fmt::{Display, Formatter};
use bevy::prelude::Component;
use crate::enums::{PlayingCardSuit, PlayingCardValue};

/// Playing card with value and suit
#[derive(Component)]
pub struct PlayingCard {
	pub(crate) value: PlayingCardValue,
	pub(crate) suit: PlayingCardSuit,
}

impl PlayingCard {
	pub fn new(value: PlayingCardValue, suit: PlayingCardSuit) -> Self {
		Self { value, suit }
	}
}

impl Display for PlayingCard {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} of {}", self.value, self.suit)
	}
}