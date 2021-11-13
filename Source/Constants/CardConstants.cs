using MobileCardGames.Source.Entities;

namespace MobileCardGames.Source.Constants
{
	/// <summary>
	/// Constants for <see cref="PlayingCard"/> (and <see cref="Deck"/>)
	/// </summary>
	public static class CardConstants
	{
		/// <summary>
		/// Texture atlas scale
		/// </summary>
		private const float Scale = 2f;

		/// <summary>
		/// Texture outside padding
		/// </summary>
		public const float Padding = 4f * Scale;

		/// <summary>
		/// Spacing between each texture
		/// </summary>
		public const float Spacing = 10f * Scale;

		/// <summary>
		/// Texture width
		/// </summary>
		public const float Width = 140f * Scale;

		/// <summary>
		/// Texture Height
		/// </summary>
		public const float Height = 190f * Scale;
	}
}