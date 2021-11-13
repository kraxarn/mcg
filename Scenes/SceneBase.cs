using Godot;
using MobileCardGames.Source;
using MobileCardGames.Source.Enums;
using MobileCardGames.Source.Exceptions;
using MobileCardGames.Source.Extensions;

namespace MobileCardGames.Scenes
{
	public abstract class SceneBase : Node
	{
		protected void Connect(string path, string signal, string method)
		{
			var error = NodeExtensions.Connect(this, path, signal, method);
#if DEBUG
			if (error != Error.Ok)
			{
				throw new ConnectException(path, signal, error);
			}
#endif
		}

		protected void GoTo(Scene scene)
		{
			var error = GetTree().ChangeScene(scene.GetPath());
#if DEBUG
			if (error != Error.Ok)
			{
				throw new SceneException(scene, error);
			}
#endif
		}
	}
}