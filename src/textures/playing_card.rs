use std::str::FromStr;
use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::entities::PlayingCard;
use crate::enums::{PlayingCardSuit, PlayingCardValue};

// This currently uses a very temporary workaround, as
// texture atlases can't be used for UI elements.
// See: https://github.com/bevyengine/bevy/pull/5070

#[derive(Resource, Default)]
pub struct PlayingCardTexture {
	handles: HashMap<PlayingCard, Handle<Image>>,
	default: Handle<Image>,
}

impl PlayingCardTexture {
	pub fn load(
		mut resource: ResMut<PlayingCardTexture>,
		asset_server: Res<AssetServer>,
	) {
		let handles = asset_server
			.load_folder("textures/cards/fronts")
			.unwrap();

		for handle in handles {
			let handle = handle.typed();
			let handle_path = asset_server.get_handle_path(&handle).unwrap();

			let path = handle_path.path()
				.file_stem().unwrap()
				.to_str().unwrap();

			if path == "joker" {
				resource.default = handle;
				continue;
			}

			let parts: Vec<&str> = path
				.split('_')
				.collect();

			let Ok(suit) = PlayingCardSuit::from_str(parts[0]) else {
				panic!("Unknown suit: {}", parts[0]);
			};

			let Ok(value) = PlayingCardValue::from_str(parts[1]) else {
				panic!("Unknown value: {}", parts[1]);
			};

			let card = PlayingCard::new(value, suit);
			resource.handles.insert(card, handle);
		}
	}

	pub fn card(&self, card: &PlayingCard) -> UiImage {
		if let Some(handle) = self.handles.get(card) {
			UiImage(handle.clone())
		} else {
			panic!("Unable to find texture for: {}", &card);
		}
	}

	pub fn ids(&self) -> Vec<HandleId> {
		self.handles.values()
			.map(|handle| handle.id())
			.collect()
	}
}
