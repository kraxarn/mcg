using MobileCardGames.Source.Enums;

namespace MobileCardGames.Source.Extensions
{
	public static class PlayingCardSuitExtensions
	{
		public static string GetName(this PlayingCardSuit suit) =>
			suit.ToString().ToLower();
	}
}