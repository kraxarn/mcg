using System.Linq;
using MobileCardGames.Shared.Entities;
using NUnit.Framework;

namespace MobileCardGames.Tests.Entities
{
	[TestFixture]
	public class DeckTests
	{
		[Test]
		public void CanGenerateDeck()
		{
			var deck = new Deck();
			Assert.AreEqual(Deck.Size, deck.Count);
		}

		[TestCase(false, ExpectedResult = true)]
		[TestCase(true, ExpectedResult = false)]
		public bool CanShuffleDeck(bool shuffle)
		{
			var sorted = new Deck();

			var shuffled = new Deck();
			if (shuffle)
			{
				shuffled.Shuffle();
			}

			Assert.AreEqual(sorted.Count, shuffled.Count);

			return sorted
				.DrawAll()
				.All(c1 => c1.Equals(shuffled.Draw()));
		}
	}
}