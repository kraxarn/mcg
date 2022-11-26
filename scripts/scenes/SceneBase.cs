using Godot;
using mcg.shared.enums;
using mcg.shared.extensions;

public abstract partial class SceneBase : Node
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
		var error = GetTree().ChangeSceneToFile(scene.GetPath());

#if DEBUG
		if (error != Error.Ok)
		{
			throw new SceneException(scene, error);
		}
#endif
	}
}