import type PlayingCardSuite from "$lib/enums/PlayingCardSuit"
import type PlayingCardValue from "$lib/enums/PlayingCardValue"

export default class PlayingCard {
	public suit: PlayingCardSuite
	public value: PlayingCardValue

	constructor(suit: PlayingCardSuite, value: PlayingCardValue) {
		this.suit = suit
		this.value = value
	}

	toString(): string {
		return `${this.value} of ${this.suit}`
	}
}
