using System;
using Godot;
using MobileCardGames.Source;

public class Menu : Node
{
	public override void _Ready()
	{
		this.Connect("Background/VBox/DeckButton", Signal.Pressed, nameof(GoToDeck));
		this.Connect("Background/VBox/ScrollButton", Signal.Pressed, nameof(GoToScroll));
		this.Connect("Background/VBox/InputButton", Signal.Pressed, nameof(GoToInput));
		this.Connect("Background/VBox/StorageButton", Signal.Pressed, nameof(GoToStorage));
		this.Connect("Background/VBox/BackButton", Signal.Pressed, nameof(Quit));
	}

	private void GoToDeck() => throw new NotImplementedException();

	private void GoToScroll() => throw new NotImplementedException();

	private void GoToInput() => throw new NotImplementedException();

	private void GoToStorage() => throw new NotImplementedException();

	private void Quit() => GetTree().Quit();
}