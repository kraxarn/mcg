using System;
using System.Collections.Generic;
using System.Linq;
using MobileCardGames.Shared.Enums;

namespace MobileCardGames.Shared.Entities
{
	public class Deck
	{
		public Deck()
		{
			cards = new Stack<PlayingCard>(Size);
			Reset();
		}

		/// <summary>
		/// Size of full deck
		/// </summary>
		public const int Size = 52;

		private readonly Stack<PlayingCard> cards;

		/// <summary>
		/// Number of cards currently in the deck
		/// </summary>
		public int Count => cards.Count;

		/// <summary>
		/// Empty current deck and fill it with new cards
		/// </summary>
		public void Reset()
		{
			cards.Clear();
			foreach (PlayingCardSuit suit in Enum.GetValues(typeof(PlayingCardSuit)))
			{
				foreach (PlayingCardValue value in Enum.GetValues(typeof(PlayingCardValue)))
				{
					cards.Push(new PlayingCard(value, suit));
				}
			}
		}

		/// <summary>
		/// Draw a card from the deck
		/// </summary>
		public PlayingCard? Draw()
		{
			return cards.Any()
				? cards.Pop()
				: default;
		}

		/// <summary>
		/// Draw cards until the deck is empty
		/// </summary>
		public IEnumerable<PlayingCard> DrawAll()
		{
			while (cards.Any())
			{
				yield return cards.Pop();
			}
		}

		/// <summary>
		/// Shuffle deck with specified randomizer
		/// </summary>
		private void Shuffle(Random random)
		{
			var allCards = DrawAll().ToList();

			while (allCards.Any())
			{
				var i = random.Next(allCards.Count);
				cards.Push(allCards[i]);
				allCards.RemoveAt(i);
			}
		}

		/// <summary>
		/// Shuffle remaining cards in deck
		/// </summary>
		public void Shuffle()
		{
			Shuffle(new Random());
		}
	}
}