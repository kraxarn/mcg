impl super::Deck {
	/// Get a 52 card deck of playing cards
	pub fn new() -> Self {
		let mut cards: std::vec::Vec<super::PlayingCard> = std::vec::Vec::with_capacity(52_usize);

		use strum::IntoEnumIterator;
		for suit in super::playing_card::Suit::iter() {
			for value in super::playing_card::Value::iter() {
				cards.push(super::PlayingCard::new(value, suit));
			}
		}

		Self { cards }
	}

	/// Shuffle deck
	pub fn shuffle(&mut self) {
		fastrand::shuffle(&mut self.cards);
	}

	/// Draw a card, removing it from the deck.
	/// Returns [`None`] if deck is empty
	pub fn draw(&mut self) -> std::option::Option<super::PlayingCard> {
		self.cards.pop()
	}
}
