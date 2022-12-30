use strum_macros::{Display, EnumIter};

#[derive(Copy, Clone, Display, EnumIter)]
pub enum PlayingCardValue {
	#[strum(serialize = "ace")]
	Ace,

	#[strum(serialize = "2")]
	Two,

	#[strum(serialize = "3")]
	Three,

	#[strum(serialize = "4")]
	Four,

	#[strum(serialize = "5")]
	Five,

	#[strum(serialize = "6")]
	Six,

	#[strum(serialize = "7")]
	Seven,

	#[strum(serialize = "8")]
	Eight,

	#[strum(serialize = "9")]
	Nine,

	#[strum(serialize = "10")]
	Ten,

	#[strum(serialize = "jack")]
	Jack,

	#[strum(serialize = "queen")]
	Queen,

	#[strum(serialize = "king")]
	King,
}