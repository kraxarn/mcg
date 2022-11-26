using System;
using Godot;

public class ConnectException : Exception
{
	public ConnectException(string path, string signal, Error error)
		: base($"Failed to connect {signal} to {path}: {error}")
	{
	}
}