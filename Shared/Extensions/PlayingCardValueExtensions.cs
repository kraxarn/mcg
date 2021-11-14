using MobileCardGames.Shared.Enums;

namespace MobileCardGames.Shared.Extensions
{
	public static class PlayingCardValueExtensions
	{
		public static string GetName(this PlayingCardValue value)
		{
			return (value >= PlayingCardValue.Two && value <= PlayingCardValue.Ten
					? ((int)value).ToString()
					: value.ToString())
				.ToLower();
		}
	}
}