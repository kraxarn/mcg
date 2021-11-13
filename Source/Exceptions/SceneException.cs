using System;
using Godot;
using MobileCardGames.Source.Enums;

namespace MobileCardGames.Source.Exceptions
{
	/// <summary>
	/// Exception thrown when failed to go to a new scene
	/// </summary>
	public class SceneException : Exception
	{
		public SceneException(Scene scene, Error error)
			: base($"Failed to go to {scene}: {error}")
		{
		}
	}
}