using System;
using Godot;

namespace MobileCardGames.Exceptions
{
	/// <summary>
	/// Exception thrown when an event failed to connect to a code
	/// </summary>
	public class ConnectException : Exception
	{
		public ConnectException(string path, string signal, Error error)
			: base($"Failed to connect {signal} to {path}: {error}")
		{
		}
	}
}