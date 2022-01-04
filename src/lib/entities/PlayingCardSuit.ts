export default class PlayingCardSuit {
	private suite: string

	private constructor() {}

	private getSuits(): string[] {
		return ["spades", "clubs", "diamonds", "hearts"]
	}
}
