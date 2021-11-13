using MobileCardGames.Scenes;
using MobileCardGames.Source;
using MobileCardGames.Source.Enums;

public class Deck : SceneBase
{
	public override void _Ready()
	{
		Connect("BackButton", Signal.Pressed, nameof(GoBack));
	}

	private void GoBack() => GoTo(Scene.DevMenu);
}