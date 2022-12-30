use bevy::asset::{AssetServer, Handle, HandleId};
use bevy::prelude::{Res, ResMut, Resource};
use bevy::text::{Font, TextStyle};
use crate::colors;

#[derive(Resource, Default)]
pub struct DefaultFont {
	font: Handle<Font>,
}

impl DefaultFont {
	pub fn load(mut resource: ResMut<DefaultFont>, asset_server: Res<AssetServer>) {
		resource.font = asset_server.load("fonts/FiraSans-Regular.ttf");
	}

	pub fn style(&self) -> TextStyle {
		TextStyle {
			font: self.font.clone(),
			font_size: 64.0,
			color: colors::FOREGROUND,
		}
	}

	pub fn id(&self) -> HandleId {
		self.font.id()
	}
}