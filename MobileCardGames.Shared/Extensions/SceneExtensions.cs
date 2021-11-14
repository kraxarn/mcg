using System;
using MobileCardGames.Shared.Enums;

namespace MobileCardGames.Shared.Extensions
{
	public static class SceneExtensions
	{
		public static string GetPath(this Scene scene)
		{
			return scene switch
			{
				Scene.DevMenu => "res://Scenes/Dev/Menu.tscn",
				Scene.DevDeck => "res://Scenes/Dev/Deck.tscn",
				Scene.DevScroll => "res://Scenes/Dev/Scroll.tscn",
				Scene.DevInput => "res://Scenes/Dev/Input.tscn",
				Scene.DevStorage => "res://Scenes/Dev/Storage.tscn",
				_ => throw new ArgumentOutOfRangeException(nameof(scene), scene, null),
			};
		}
	}
}