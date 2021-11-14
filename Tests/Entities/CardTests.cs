using MobileCardGames.Shared.Entities;
using MobileCardGames.Shared.Enums;
using NUnit.Framework;

namespace MobileCardGames.Tests.Entities
{
	[TestFixture]
	public class CardTests
	{
		[Test]
		public void SameCardsAreEqual()
		{
			var card1 = new PlayingCard(PlayingCardValue.Ace, PlayingCardSuit.Hearts);
			var card2 = new PlayingCard(PlayingCardValue.Ace, PlayingCardSuit.Hearts);

			Assert.AreEqual(card1, card2);
			Assert.AreEqual(card1.GetHashCode(), card2.GetHashCode());
		}

		[Test]
		public void DifferentCardsAreNotEqual()
		{
			var card1 = new PlayingCard(PlayingCardValue.Ace, PlayingCardSuit.Hearts);
			var card2 = new PlayingCard(PlayingCardValue.Two, PlayingCardSuit.Hearts);
			var card3 = new PlayingCard(PlayingCardValue.Ace, PlayingCardSuit.Diamonds);

			Assert.AreNotEqual(card1, card2);
			Assert.AreNotEqual(card1, card3);

			Assert.AreNotEqual(card1.GetHashCode(), card2.GetHashCode());
			Assert.AreNotEqual(card1.GetHashCode(), card3.GetHashCode());
		}
	}
}