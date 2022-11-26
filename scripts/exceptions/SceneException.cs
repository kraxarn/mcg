using System;
using Godot;
using mcg.shared.enums;

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