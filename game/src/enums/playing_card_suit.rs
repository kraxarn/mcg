use strum_macros::{Display, EnumIter};

#[derive(Copy, Clone, Display, EnumIter)]
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