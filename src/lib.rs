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
		.add_event::<DrawCardEvent>()
		// Setup textures
		.add_system_set(SystemSet::on_enter(AppState::SetupTextures)
			.with_system(assets::setup_textures))
		.add_system_set(SystemSet::on_update(AppState::SetupTextures)
			.with_system(assets::check_textures))
		// Setup fonts
		.add_system_set(SystemSet::on_enter(AppState::SetupFonts)
			.with_system(assets::setup_fonts))
		.add_system_set(SystemSet::on_update(AppState::SetupFonts)
			.with_system(assets::check_fonts))
		// Ready
		.add_system_set(SystemSet::on_enter(AppState::Ready)
			.with_system(setup_camera)
			.with_system(show_example_card))
		.add_system_set(SystemSet::on_update(AppState::Ready)
			.with_system(update_card_texture)
			.with_system(update_draw_card_button))
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
struct PlayingCardSprite;

#[derive(Component)]
struct PlayingCardText;

struct DrawCardEvent(PlayingCard);

fn show_example_card(
	mut commands: Commands,
	card_texture: Res<PlayingCardTexture>,
	default_font: Res<DefaultFont>,
	mut deck: ResMut<Deck>,
	mut draw_card_event: EventWriter<DrawCardEvent>,
) {
	let mut sprite = card_texture.joker();
	let position = Vec3::new(0.0, 120.0, 0.0);
	sprite.transform.translation = position;

	commands.spawn((sprite, PlayingCardSprite));

	commands.spawn((
		Text2dBundle {
			text: Text::from_section(String::new(), default_font.style())
				.with_alignment(TextAlignment::CENTER),
			transform: Transform::from_xyz(0.0, position.y - 260.0, 0.0),
			..default()
		},
		PlayingCardText,
	));

	commands.spawn(NodeBundle {
		style: Style {
			size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
			position: UiRect::new(
				Val::Px(0.0),
				Val::Px(0.0),
				Val::Percent(100.0),
				Val::Px(180.0),
			),
			align_items: AlignItems::FlexEnd,
			justify_content: JustifyContent::Center,
			..default()
		},
		..default()
	}).with_children(|parent| {
		parent.spawn(ButtonBundle {
			style: Style {
				size: Size::new(Val::Percent(75.0), Val::Px(100.0)),
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				..default()
			},
			background_color: colors::BUTTON.into(),
			..default()
		}).with_children(|parent| {
			parent.spawn(TextBundle::from_sections([
				TextSection::new("Draw card ", default_font.style()),
				TextSection::new("1", default_font.style()),
			]));
		});
	});

	deck.reset();
	deck.shuffle();
	draw_card_event.send(DrawCardEvent {
		0: deck.draw().unwrap(),
	});
}

fn update_card_texture(
	mut card_sprites: Query<&mut TextureAtlasSprite, With<PlayingCardSprite>>,
	mut texts: Query<&mut Text, With<PlayingCardText>>,
	mut draw_card_event: EventReader<DrawCardEvent>,
) {
	if let Some(event) = draw_card_event.iter().last() {
		texts.single_mut().sections[0].value = event.0.to_string();
		card_sprites.single_mut().index = event.0.sprite_index();
	}
}

fn update_draw_card_button(
	mut interactions: Query<(&Interaction, &mut BackgroundColor, &Children),
		(Changed<Interaction>, With<Button>)>,
	mut texts: Query<&mut Text>,
	mut deck: ResMut<Deck>,
	mut draw_card_event: EventWriter<DrawCardEvent>,
) {
	for (interaction, mut color, children) in &mut interactions {
		*color = match *interaction {
			Interaction::Clicked => colors::BUTTON_CLICKED,
			Interaction::Hovered => {
				if color.0 == colors::BUTTON_CLICKED {
					let mut text = texts.get_mut(children[0]).unwrap();
					if let Some(card) = deck.draw() {
						text.sections[1].value = (Deck::MAX - deck.len()).to_string();
						draw_card_event.send(DrawCardEvent { 0: card });
					} else {
						text.sections[0].value = String::from("Deck empty");
						text.sections[1].value = String::new();
					}
				}
				colors::BUTTON_HOVERED
			},
			Interaction::None => colors::BUTTON,
		}.into();
	}
}