use bevy::prelude::Component;

#[derive(Component, Copy, Clone)]
pub enum ButtonId {
	// dev_menu_scene
	GoToDevCard,
	GoToBlackjack,
	// dev_card_Scene
	DrawCard,
	GoToDevMenu,
	// blackjack_scene
	BlackjackHit,
	BlackjackStand,
	//BlackjackDoubleDown,
	//BlackjackSplit,
}