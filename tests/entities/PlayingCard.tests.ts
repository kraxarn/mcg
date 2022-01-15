import { jest } from "@jest/globals"

import PlayingCard from "../../src/lib/entities/PlayingCard"
import PlayingCardSuit from "../../src/lib/enums/PlayingCardSuit"
import PlayingCardValue from "../../src/lib/enums/PlayingCardValue"

test("same cards are equal", () => {
	const card1 = new PlayingCard(PlayingCardSuit.Hearts, PlayingCardValue.Ace)
	const card2 = new PlayingCard(PlayingCardSuit.Hearts, PlayingCardValue.Ace)

	expect(card1).toBe(card2)
})
