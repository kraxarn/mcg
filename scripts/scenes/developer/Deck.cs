using Godot;
using mcg.shared.enums;

public partial class Deck : SceneBase
{
	private mcg.shared.entities.Deck? deck;

	private Button? drawCardButton;

	private CardTexture? cardScene;
	private Label? cardLabel;

	public override void _Ready()
	{
		Connect("container_buttons/button_back", Signals.Pressed, nameof(GoBack));

		drawCardButton = GetNode<Button>("container_buttons/button_draw_card");
		drawCardButton.Connect(Signals.Pressed, new Callable(this, nameof(DrawCard)));

		cardScene = GetNode<CardTexture>("container_card/texture_card");
		cardLabel = GetNode<Label>("container_card/label_card");

		deck = new mcg.shared.entities.Deck();
		deck.Shuffle();

		DrawCard();
	}

	private void GoBack() =>
		GoTo(Scene.DevMenu);

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
			drawCardButton.Text = $"Draw card {mcg.shared.entities.Deck.Size - deck?.Count + 1}";
		}
	}
}