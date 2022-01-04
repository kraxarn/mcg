export default class PlayingCardValue {
	/** Minimum value: Ace */
	private readonly minimum: number = 1

	/** Maximum value: King */
	private readonly maximum: number = 13

	/** Current value */
	private currentValue: number

	constructor(value: number) {
		this.currentValue = value
	}

	public get value(): number {
		return this.currentValue
	}

	public set value(value: number) {
		if (value < this.minimum || value > this.maximum) {
			throw `${value} is not a valid value for PlayingCardValue`
		}
		this.currentValue = value
	}

	public *allValues(): Generator<PlayingCardValue> {
		for (let i = this.minimum; i <= this.maximum; i++) {
			yield new PlayingCardValue(i)
		}
	}

	public toString(): string {
		switch (this.currentValue) {
			case 1:
				return "ace"
			case 2:
				return "two"
			case 3:
				return "three"
			case 4:
				return "four"
			case 5:
				return "five"
			case 6:
				return "six"
			case 7:
				return "seven"
			case 8:
				return "eight"
			case 9:
				return "nine"
			case 10:
				return "ten"
			case 11:
				return "jack"
			case 12:
				return "queen"
			case 13:
				return "king"
		}
	}
}
