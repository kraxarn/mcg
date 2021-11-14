using MobileCardGames.Shared.Entities;
using Xunit;

namespace MobileCardGames.Tests.Entities
{
	public class DeckTests
	{
		[Fact]
		public void CanGenerateDeck()
		{
			var deck = new Deck();
			Assert.Equal(52, deck.Count);
		}

		[Fact]
		public void CanShuffleDeck()
		{
			var sorted = new Deck();

			var shuffled = new Deck();
			//shuffled.Shuffle();

			Assert.Equal(sorted.Count, shuffled.Count);

			while (sorted.Count > 0 && shuffled.Count > 0)
			{
				var sortedCard = sorted.Draw();
				var shuffledCard = shuffled.Draw();

				Assert.NotEqual(sortedCard, shuffledCard);
			}
		}
	}
}