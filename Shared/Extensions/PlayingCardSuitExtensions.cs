using mcg.shared.enums;

namespace mcg.shared.extensions;

public static class PlayingCardSuitExtensions
{
	public static string GetName(this PlayingCardSuit suit) =>
		suit.ToString().ToLower();
}