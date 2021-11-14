using System;
using MobileCardGames.Shared.Enums;
using MobileCardGames.Shared.Extensions;

namespace MobileCardGames.Shared.Entities
{
	public class PlayingCard : IEquatable<PlayingCard>
	{
		public PlayingCard(PlayingCardValue value, PlayingCardSuit suit)
		{
			Value = value;
			Suit = suit;
		}

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
}