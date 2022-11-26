using System;
using mcg.shared.enums;
using mcg.shared.extensions;

namespace mcg.shared.entities;

public readonly struct PlayingCard : IEquatable<PlayingCard>
{
	public PlayingCard(PlayingCardValue value, PlayingCardSuit suit)
	{
		Value = value;
		Suit = suit;
	}

	/// <summary>
	/// Ace of spaces
	/// </summary>
	public static readonly PlayingCard Default = new(PlayingCardValue.Ace, PlayingCardSuit.Spades);

	public PlayingCardValue Value { get; }

	public PlayingCardSuit Suit { get; }

	public override string ToString() =>
		$"{Value.GetName()} of {Suit.GetName()}";

	public bool Equals(PlayingCard other)
	{
		return Value == other.Value
			&& Suit == other.Suit;
	}

	public override bool Equals(object? obj)
	{
		return obj is PlayingCard playingCard
			&& Equals(playingCard);
	}

	public override int GetHashCode()
	{
		unchecked
		{
			return ((int)Value * 397) ^ (int)Suit;
		}
	}
}