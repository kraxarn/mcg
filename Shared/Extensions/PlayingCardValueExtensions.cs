using mcg.shared.enums;

namespace mcg.shared.extensions;

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