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
			Assert.Equal(Deck.Size, deck.Count);
		}

		[Fact]
		public void CanShuffleDeck()
		{
			var sorted = new Deck();

			var shuffled = new Deck();
			// "Random" seed that shuffles entire deck every time
			shuffled.Shuffle(1248);

			Assert.Equal(sorted.Count, shuffled.Count);

			while (sorted.Count > 0 && shuffled.Count > 0)
			{
				var sortedCard = sorted.Draw();
				var shuffledCard = shuffled.Draw();

				Assert.NotEqual(sortedCard, shuffledCard);
			}

			Assert.Equal(0, sorted.Count);
			Assert.Equal(0, shuffled.Count);
		}
	}
}