using Godot;
using mcg.shared.entities;

public partial class CardTexture : TextureRect
{
	private PlayingCard? playingCard;

	/// <summary>
	/// Current card
	/// </summary>
	public PlayingCard PlayingCard
	{
		get => playingCard ?? PlayingCard.Default;
		set
		{
			if (Texture is AtlasTexture atlasTexture)
			{
				var region = atlasTexture.Region;
				region.Position = value.GetAtlasPosition();
				atlasTexture.Region = region;
			}

			playingCard = value;
		}
	}
}