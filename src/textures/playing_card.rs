use bevy::asset::HandleId;
use bevy::prelude::*;
use strum::IntoEnumIterator;
use crate::entities::PlayingCard;
use crate::enums::{PlayingCardSuit, PlayingCardValue};

#[derive(Resource, Default)]
pub struct PlayingCardTexture {
	texture: Handle<Image>,
	atlas: Handle<TextureAtlas>,
}

impl PlayingCardTexture {
	pub fn load(
		mut resource: ResMut<PlayingCardTexture>,
		asset_server: Res<AssetServer>,
		mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	) {
		let texture = asset_server.load("textures/playing_cards.png");

		let scale = 2.0;
		let tile_size = Vec2::new(140.0, 190.0) * scale;
		let columns = Self::atlas_columns();
		let rows = Self::atlas_rows();
		let padding = Some(Vec2::new(10.0, 10.0) * scale);
		let offset = Some(Vec2::new(4.0, 4.0) * scale);
		let atlas = TextureAtlas::from_grid(texture, tile_size, columns, rows, padding, offset);

		resource.texture = atlas.texture.clone();
		resource.atlas = texture_atlases.add(atlas);
	}

	fn atlas_columns() -> usize {
		PlayingCardValue::iter().len() + 1 // Joker
	}

	fn atlas_rows() -> usize {
		PlayingCardSuit::iter().len()
	}

	pub fn sprite(&self, card: &PlayingCard) -> SpriteSheetBundle {
		let row = card.suit as usize;
		let column = card.value as usize;
		let index = column + (row * Self::atlas_columns());

		SpriteSheetBundle {
			sprite: TextureAtlasSprite::new(index),
			texture_atlas: self.atlas.clone(),
			..default()
		}
	}

	pub fn id(&self) -> HandleId {
		self.texture.id()
	}
}
