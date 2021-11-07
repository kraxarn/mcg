using System;
using System.ComponentModel;
using System.Reflection;

namespace MobileCardGames.Source
{
	public static class EnumExtensions
	{
		public static string GetDescription(this Enum value)
		{
			var fieldInfo = value.GetType().GetField(value.ToString());
			var attribute = fieldInfo.GetCustomAttribute(typeof(DescriptionAttribute));
			return ((DescriptionAttribute)attribute).Description;
		}
	}
}