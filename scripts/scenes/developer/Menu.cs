using mcg.shared.enums;

public partial class Menu : SceneBase
{
	public override void _Ready()
	{
		Connect("container/button_deck", Signals.Pressed, nameof(GoToDeck));
		Connect("container/button_scroll", Signals.Pressed, nameof(GoToScroll));
		Connect("container/button_input", Signals.Pressed, nameof(GoToInput));
		Connect("container/button_storage", Signals.Pressed, nameof(GoToStorage));
		Connect("container/button_tcp", Signals.Pressed, nameof(GoToTcp));
		Connect("container/button_back", Signals.Pressed, nameof(Quit));
	}

	private void GoToDeck() => GoTo(Scene.DevDeck);

	private void GoToScroll() => GoTo(Scene.DevScroll);

	private void GoToInput() => GoTo(Scene.DevInput);

	private void GoToStorage() => GoTo(Scene.DevStorage);

	private void GoToTcp() => GoTo(Scene.DevTcp);

	private void Quit() => GetTree().Quit();
}