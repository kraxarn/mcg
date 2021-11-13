using System;
using System.Collections.Generic;
using System.Linq;
using MobileCardGames.Source.Enums;

namespace MobileCardGames.Source.Entities
{
	public class Deck
	{
		public Deck()
		{
			cards = new Stack<PlayingCard>(DeckSize);
		}

		private const int DeckSize = 52;

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
		private IEnumerable<PlayingCard> DrawAll()
		{
			while (cards.Any())
			{
				yield return cards.Pop();
			}
		}

		/// <summary>
		/// Shuffle remaining cards in deck
		/// </summary>
		public void Shuffle()
		{
			var random = new Random();
			var shuffled = DrawAll().ToArray();

			for (var i = 0; i < shuffled.Length - 1; i++)
			{
				var pos = random.Next(0, shuffled.Length);
				(shuffled[i], shuffled[pos]) = (shuffled[pos], shuffled[i]);
			}

			foreach (var card in shuffled)
			{
				cards.Push(card);
			}
		}
	}
}