using Godot;

public static class NodeExtensions
{
	public static Error Connect(this Node node, string path, string signal, string method)
	{
		return node.GetNode<BaseButton>(path)
			.Connect(signal, new Callable(node, method));
	}
}