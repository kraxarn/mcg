use strum_macros::{Display, EnumIter, EnumString};

#[derive(Copy, Clone, Display, EnumIter, EnumString, Hash, Eq, PartialEq)]
pub enum PlayingCardSuit {
	#[strum(serialize = "spades")]
	Spades,

	#[strum(serialize = "clubs")]
	Clubs,

	#[strum(serialize = "diamonds")]
	Diamonds,

	#[strum(serialize = "hearts")]
	Hearts,
}