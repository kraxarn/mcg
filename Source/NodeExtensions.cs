using System.Collections.Generic;
using System.Linq;
using Godot;

namespace MobileCardGames.Source
{
	public static class NodeExtensions
	{
		public static Error Connect(this Node node, string path, Signal signal, string method)
		{
			GD.Print("Connected \"", signal.GetDescription(), "\" to: ", path);
			return node.GetNode<BaseButton>(path)
				.Connect(signal.GetDescription(), node, method);
		}

		public static IList<Error> Connect(this Node node, string scope, Signal signal,
			IEnumerable<(string path, string method)> methods)
		{
			return methods
				.Select(m => node.Connect($"{scope}/{m.path}", signal, m.method))
				.ToList();
		}
	}
}