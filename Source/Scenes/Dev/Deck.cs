using Godot;
using MobileCardGames.Constants;
using MobileCardGames.Scenes;
using MobileCardGames.Shared.Enums;

public class Deck : SceneBase
{
	private MobileCardGames.Shared.Entities.Deck? deck;

	private Button? drawCardButton;

	private CardTexture? cardScene;
	private Label? cardLabel;

	public override void _Ready()
	{
		Connect("BackButton", SignalConstants.Pressed, nameof(GoBack));

		drawCardButton = GetNode<Button>("DrawCard");
		drawCardButton.Connect(SignalConstants.Pressed, this, nameof(DrawCard));

		cardScene = GetNode<CardTexture>("Container/CardTexture");
		cardLabel = GetNode<Label>("Container/CardName");

		deck = new MobileCardGames.Shared.Entities.Deck();
		deck.Shuffle();

		DrawCard();
	}

	private void GoBack() => GoTo(Scene.DevMenu);

	private void DrawCard()
	{
		var card = deck?.Draw();
		if (card == null)
		{
			if (drawCardButton != null)
			{
				drawCardButton.Text = "Deck empty";
			}

			return;
		}

		if (cardLabel != null)
		{
			cardLabel.Text = card.ToString();
		}

		if (cardScene != null)
		{
			cardScene.PlayingCard = card.Value;
		}

		if (drawCardButton != null)
		{
			drawCardButton.Text = $"Draw card {MobileCardGames.Shared.Entities.Deck.Size - deck?.Count + 1}";
		}
	}
}