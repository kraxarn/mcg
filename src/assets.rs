use bevy::asset::LoadState;
use bevy::prelude::*;
use crate::AppState;
use crate::fonts::DefaultFont;
use crate::textures::PlayingCardTexture;

pub fn setup_textures(
	asset_server: Res<AssetServer>,
	texture_atlases: ResMut<Assets<TextureAtlas>>,
	card_texture: ResMut<PlayingCardTexture>,
) {
	PlayingCardTexture::load(card_texture, asset_server, texture_atlases);
}

pub fn setup_fonts(
	asset_server: Res<AssetServer>,
	default_font: ResMut<DefaultFont>,
) {
	DefaultFont::load(default_font, asset_server);
}

pub fn check_textures(
	mut state: ResMut<State<AppState>>,
	asset_server: Res<AssetServer>,
	card_texture: Res<PlayingCardTexture>,
) {
	let handles = [
		card_texture.id(),
	];
	if asset_server.get_group_load_state(handles) == LoadState::Loaded {
		state.set(AppState::SetupFonts).unwrap();
	}
}

pub fn check_fonts(
	mut state: ResMut<State<AppState>>,
	asset_server: Res<AssetServer>,
	default_font: Res<DefaultFont>,
) {
	let handles = [
		default_font.id(),
	];
	if asset_server.get_group_load_state(handles) == LoadState::Loaded {
		state.set(AppState::Ready).unwrap();
	}
}