use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::settings::{WgpuSettings, WgpuSettingsPriority};
use crate::entities::{Deck, PlayingCard};
use crate::fonts::DefaultFont;
use crate::textures::PlayingCardTexture;

mod colors;
mod enums;
mod entities;
mod textures;
mod fonts;
mod assets;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum AppState {
	SetupTextures,
	SetupFonts,
	Ready,
}

pub fn run() {
	App::new()
		.init_resource::<PlayingCardTexture>()
		.init_resource::<DefaultFont>()
		.insert_resource(WgpuSettings {
			priority: WgpuSettingsPriority::Compatibility,
			..default()
		})
		.add_plugins(DefaultPlugins
			.set(WindowPlugin {
				window: WindowDescriptor {
					title: format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
					width: 540.0,
					height: 960.0,
					..default()
				},
				..default()
			}))
		.add_state(AppState::SetupTextures)
		.add_system_set(SystemSet::on_enter(AppState::SetupTextures)
			.with_system(assets::setup_textures))
		.add_system_set(SystemSet::on_update(AppState::SetupTextures)
			.with_system(assets::check_textures))
		.add_system_set(SystemSet::on_enter(AppState::SetupFonts)
			.with_system(assets::setup_fonts))
		.add_system_set(SystemSet::on_update(AppState::SetupFonts)
			.with_system(assets::check_fonts))
		.add_system_set(SystemSet::on_enter(AppState::Ready)
			.with_system(setup_camera)
			.with_system(show_example_card))
		.add_system_set(SystemSet::on_update(AppState::Ready)
			.with_system(update_card_title))
		.run();
}

fn setup_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle {
		camera_2d: Camera2d {
			clear_color: ClearColorConfig::Custom(colors::BACKGROUND),
			..default()
		},
		..default()
	});
}

#[derive(Component)]
struct PlayingCardText;

fn show_example_card(
	mut commands: Commands,
	card_texture: Res<PlayingCardTexture>,
	default_font: Res<DefaultFont>,
) {
	let mut deck = Deck::new();
	deck.shuffle();
	let card = deck.draw().unwrap();
	let sprite = card_texture.sprite(&card);

	commands.spawn((sprite, card));
	commands.spawn((
		Text2dBundle {
			text: Text::from_sections([
				TextSection::new(String::new(), default_font.style()),
				TextSection::new(format!("\n{} cards", deck.len()), default_font.style()),
			]).with_alignment(TextAlignment::CENTER),
			transform: Transform::from_xyz(0.0, -300.0, 0.0),
			..default()
		},
		PlayingCardText,
	));
}

fn update_card_title(
	cards: Query<&PlayingCard>,
	mut texts: Query<&mut Text, With<PlayingCardText>>,
) {
	let card = cards.single();
	texts.single_mut().sections[0].value = card.to_string();
}