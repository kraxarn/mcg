using MobileCardGames.Shared.Enums;
using MobileCardGames.Shared.Extensions;

namespace MobileCardGames.Shared.Entities
{
	public class PlayingCard
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
	}
}