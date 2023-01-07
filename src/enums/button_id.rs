use bevy::prelude::Component;

#[derive(Component, Copy, Clone)]
pub enum ButtonId {
	// dev_menu_scene
	GoToDevCard,
	GoToBlackJack,
	// dev_card_Scene
	DrawCard,
	GoToDevMenu,
}