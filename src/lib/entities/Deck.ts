import PlayingCard from "$lib/entities/PlayingCard"
import PlayingCardSuite from "$lib/enums/PlayingCardSuit"
import PlayingCardValue from "$lib/enums/PlayingCardValue"

export default class Deck {
	private cards: PlayingCard[]

	constructor() {
		this.reset()
	}

	/**
	 * Put all cards back in the deck, in order
	 */
	public reset(): void {
		this.cards = []

		for (
			let suit = PlayingCardSuite.Spades;
			i <= PlayingCardSuite.Hearts;
			i++
		) {
			for (
				let value = PlayingCardValue.Ace;
				i <= PlayingCardValue.King;
				i++
			) {
				this.cards.push(new PlayingCard(suit, value))
			}
		}
	}

	/**
	 * Shuffle deck
	 */
	public shuffle(): void {
		for (let i = this.cards.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1))
			;[this.cards[i], this.cards[j]] = [this.cards[j], this.cards[i]]
		}
	}

	/**
	 * Draw a card, removing it from the deck,
	 * returns undefined if deck is empty
	 */
	public draw(): PlayingCard | undefined {
		return this.cards.pop()
	}

	/**
	 * Number of cards currently in the deck
	 */
	public get length(): number {
		return this.cards.length
	}
}
