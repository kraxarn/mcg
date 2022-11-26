using System;
using mcg.shared.enums;

namespace mcg.shared.extensions;

public static class SceneExtensions
{
	public static string GetPath(this Scene scene)
	{
		return scene switch
		{
			Scene.DevMenu => "res://scenes/developer/menu.tscn",
			Scene.DevDeck => "res://scenes/developer/deck.tscn",
			Scene.DevScroll => "res://scenes/developer/scroll.tscn",
			Scene.DevInput => "res://scenes/developer/input.tscn",
			Scene.DevStorage => "res://scenes/developer/storage.tscn",
			Scene.DevTcp => "res://scenes/developer/tcp.tscn",
			_ => throw new ArgumentOutOfRangeException(nameof(scene), scene, null),
		};
	}
}