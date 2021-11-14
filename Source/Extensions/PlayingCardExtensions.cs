using Godot;
using MobileCardGames.Shared.Constants;
using MobileCardGames.Shared.Entities;

namespace MobileCardGames.Extensions
{
	public static class PlayingCardExtensions
	{
		public static Vector2 GetAtlasPosition(this PlayingCard playingCard)
		{
			var value = (float)playingCard.Value - 1;
			var suit = (float)playingCard.Suit;

			return new Vector2
			{
				x = CardConstants.Padding
					+ value * CardConstants.Width
					+ CardConstants.Spacing * value,
				y = CardConstants.Padding
					+ suit * CardConstants.Height
					+ CardConstants.Spacing * suit,
			};
		}

		public static Vector2 GetAtlasPosition(this PlayingCard? playingCard)
		{
			return playingCard is { } card
				? GetAtlasPosition(card)
				: Vector2.Zero;
		}
	}
}