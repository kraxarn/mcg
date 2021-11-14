using MobileCardGames.Constants;
using MobileCardGames.Scenes;
using MobileCardGames.Shared.Enums;

public class Deck : SceneBase
{
	public override void _Ready()
	{
		Connect("BackButton", SignalConstants.Pressed, nameof(GoBack));
	}

	private void GoBack() => GoTo(Scene.DevMenu);
}