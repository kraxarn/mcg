use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::settings::{WgpuSettings, WgpuSettingsPriority};
use crate::entities::Deck;
use crate::fonts::DefaultFont;
use crate::scenes::{DevCardScene, DevMenuScene, Scene};
use crate::textures::PlayingCardTexture;

mod colors;
mod enums;
mod entities;
mod textures;
mod fonts;
mod assets;
mod scenes;
mod events;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum AppState {
	SetupTextures,
	SetupFonts,
	Ready,
	DevCard,
}

pub fn run() {
	let mut app = App::new();

	app
		.init_resource::<PlayingCardTexture>()
		.init_resource::<DefaultFont>()
		.init_resource::<Deck>()
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
		// Button listeners
		.add_event::<events::AddTextButtonEvent>()
		.add_event::<events::ButtonClickedEvent>()
		.add_system(events::update_add_text_button)
		.add_system(events::update_button_clicked)
		// Setup textures
		.add_system_set(SystemSet::on_enter(AppState::SetupTextures)
			.with_system(assets::setup_textures))
		.add_system_set(SystemSet::on_update(AppState::SetupTextures)
			.with_system(assets::check_textures))
		// Setup fonts
		.add_system_set(SystemSet::on_enter(AppState::SetupFonts)
			.with_system(assets::setup_fonts))
		.add_system_set(SystemSet::on_update(AppState::SetupFonts)
			.with_system(assets::check_fonts));

	let scenes: [&dyn Scene; 2] = [
		&DevMenuScene,
		&DevCardScene,
	];

	for scene in scenes {
		scene.add(&mut app);

		app.add_system_set(scene
			.on_enter(SystemSet::on_enter(scene.state())));

		app.add_system_set(scene
			.on_update(SystemSet::on_update(scene.state())));
	}

	app.run()
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