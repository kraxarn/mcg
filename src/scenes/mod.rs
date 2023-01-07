use bevy::app::App;
use bevy::prelude::SystemSet;
use crate::AppState;

mod dev_card_scene;
mod dev_menu_scene;

pub use dev_card_scene::*;
pub use dev_menu_scene::DevMenuScene;

pub trait Scene {
	fn state(&self) -> AppState;
	fn add(&self, app: &mut App);

	fn on_enter(&self, system_set: SystemSet) -> SystemSet;
	fn on_update(&self, system_set: SystemSet) -> SystemSet;
	fn on_exit(&self, system_set: SystemSet) -> SystemSet;
}