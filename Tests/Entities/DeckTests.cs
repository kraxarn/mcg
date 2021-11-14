using System.Linq;
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

		[Theory]
		[InlineData(true, false)]
		[InlineData(false, true)]
		public void CanShuffleDeck(bool expected, bool shuffle)
		{
			var sorted = new Deck();

			var shuffled = new Deck();
			if (shuffle)
			{
				shuffled.Shuffle();
			}

			Assert.Equal(sorted.Count, shuffled.Count);

			Assert.Equal(expected, sorted
				.DrawAll()
				.All(c1 => c1.Equals(shuffled.Draw())));
		}
	}
}