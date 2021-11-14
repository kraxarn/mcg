using MobileCardGames.Shared.Enums;

namespace MobileCardGames.Shared.Extensions
{
	public static class PlayingCardSuitExtensions
	{
		public static string GetName(this PlayingCardSuit suit) =>
			suit.ToString().ToLower();
	}
}