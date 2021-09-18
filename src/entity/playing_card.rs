#[derive(Copy, Clone, strum_macros::Display, strum_macros::EnumIter)]
pub enum Value {
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

#[derive(Copy, Clone, strum_macros::Display, strum_macros::EnumIter)]
pub enum Suit {
	#[strum(serialize = "spades")]
	Spades,
	#[strum(serialize = "clubs")]
	Clubs,
	#[strum(serialize = "diamonds")]
	Diamonds,
	#[strum(serialize = "hearts")]
	Hearts,
}

impl super::PlayingCard {
	pub fn new(value: Value, suit: Suit) -> Self {
		Self { value, suit }
	}
}

impl std::fmt::Display for super::PlayingCard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} of {}", self.value, self.suit)
	}
}
