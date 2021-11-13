using System;
using Godot;
using MobileCardGames.Scenes;
using MobileCardGames.Source;
using MobileCardGames.Source.Enums;
using MobileCardGames.Source.Extensions;

public class Menu : SceneBase
{
	public override void _Ready()
	{
		Connect("Background/VBox/DeckButton", Signal.Pressed, nameof(GoToDeck));
		Connect("Background/VBox/ScrollButton", Signal.Pressed, nameof(GoToScroll));
		Connect("Background/VBox/InputButton", Signal.Pressed, nameof(GoToInput));
		Connect("Background/VBox/StorageButton", Signal.Pressed, nameof(GoToStorage));
		Connect("Background/VBox/BackButton", Signal.Pressed, nameof(Quit));
	}

	private void GoToDeck() => GoTo(Scene.DevDeck);

	private void GoToScroll() => GoTo(Scene.DevScroll);

	private void GoToInput() => GoTo(Scene.DevInput);

	private void GoToStorage() => GoTo(Scene.DevStorage);

	private void Quit() => GetTree().Quit();
}