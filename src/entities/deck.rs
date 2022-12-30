use strum::IntoEnumIterator;
use crate::entities::PlayingCard;
use crate::enums::{PlayingCardSuit, PlayingCardValue};

pub struct Deck {
	cards: Vec<PlayingCard>,
}

impl Deck {
	/// Get a 52 card deck of playing cards
	pub fn new() -> Self {
		let mut deck = Self {
			cards: Vec::with_capacity(52_usize),
		};

		deck.reset();
		deck
	}

	// Put all cards back in the deck, in order
	pub fn reset(&mut self) {
		self.cards.clear();

		for suit in PlayingCardSuit::iter() {
			for value in PlayingCardValue::iter() {
				self.cards.push(PlayingCard::new(value, suit));
			}
		}
	}

	/// Shuffle deck
	pub fn shuffle(&mut self) {
		fastrand::shuffle(&mut self.cards);
	}

	/// Draw a card, removing it from the deck.
	/// Returns [`None`] if deck is empty
	pub fn draw(&mut self) -> Option<PlayingCard> {
		self.cards.pop()
	}

	/// Number of cards currently in the deck
	pub fn len(&self) -> usize {
		self.cards.len()
	}
}