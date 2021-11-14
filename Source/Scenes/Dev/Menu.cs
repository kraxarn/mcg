using MobileCardGames.Constants;
using MobileCardGames.Scenes;
using MobileCardGames.Shared.Enums;

public class Menu : SceneBase
{
	public override void _Ready()
	{
		Connect("Background/VBox/DeckButton", SignalConstants.Pressed, nameof(GoToDeck));
		Connect("Background/VBox/ScrollButton", SignalConstants.Pressed, nameof(GoToScroll));
		Connect("Background/VBox/InputButton", SignalConstants.Pressed, nameof(GoToInput));
		Connect("Background/VBox/StorageButton", SignalConstants.Pressed, nameof(GoToStorage));
		Connect("Background/VBox/TcpButton", SignalConstants.Pressed, nameof(GoToTcp));
		Connect("Background/VBox/BackButton", SignalConstants.Pressed, nameof(Quit));
	}

	private void GoToDeck() => GoTo(Scene.DevDeck);

	private void GoToScroll() => GoTo(Scene.DevScroll);

	private void GoToInput() => GoTo(Scene.DevInput);

	private void GoToStorage() => GoTo(Scene.DevStorage);

	private void GoToTcp() => GoTo(Scene.DevTcp);

	private void Quit() => GetTree().Quit();
}