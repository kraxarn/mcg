using System;
using Godot;
using MobileCardGames.Source;

public class Menu : Node
{
	public override void _Ready()
	{
		this.Connect("Background/VBox", Signal.Pressed, new[]
		{
			("DeckButton", nameof(GoToDeck)),
			("ScrollButton", nameof(GoToScroll)),
			("InputButton", nameof(GoToInput)),
			("StorageButton", nameof(GoToStorage)),
			("BackButton", nameof(Quit)),
		});
	}

	private void GoToDeck() => throw new NotImplementedException();

	private void GoToScroll() => throw new NotImplementedException();

	private void GoToInput() => throw new NotImplementedException();

	private void GoToStorage() => throw new NotImplementedException();

	private void Quit() => GetTree().Quit();
}