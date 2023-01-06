use bevy::app::App;
use bevy::prelude::SystemSet;
use crate::AppState;

pub use dev_card_scene::*;

pub mod dev_card_scene;

pub trait Scene {
	fn state(&self) -> AppState;
	fn add(&self, app: &mut App);

	fn on_enter(&self, system_set: SystemSet) -> SystemSet;
	fn on_update(&self, system_set: SystemSet) -> SystemSet;
}