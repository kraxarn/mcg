using MobileCardGames.Shared.Entities;
using MobileCardGames.Shared.Enums;
using Xunit;

namespace MobileCardGames.Tests.Entities
{
	public class CardTests
	{
		[Fact]
		public void SameCardsAreEqual()
		{
			var card1 = new PlayingCard(PlayingCardValue.Ace, PlayingCardSuit.Hearts);
			var card2 = new PlayingCard(PlayingCardValue.Ace, PlayingCardSuit.Hearts);

			Assert.Equal(card1, card2);
			Assert.Equal(card1.GetHashCode(), card2.GetHashCode());
		}

		[Fact]
		public void DifferentCardsAreNotEqual()
		{
			var card1 = new PlayingCard(PlayingCardValue.Ace, PlayingCardSuit.Hearts);
			var card2 = new PlayingCard(PlayingCardValue.Two, PlayingCardSuit.Hearts);
			var card3 = new PlayingCard(PlayingCardValue.Ace, PlayingCardSuit.Diamonds);

			Assert.NotEqual(card1, card2);
			Assert.NotEqual(card1, card3);

			Assert.NotEqual(card1.GetHashCode(), card2.GetHashCode());
			Assert.NotEqual(card1.GetHashCode(), card3.GetHashCode());
		}
	}
}