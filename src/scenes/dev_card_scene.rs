use bevy::prelude::*;
use crate::AppState;
use crate::entities::{Deck, PlayingCard};
use crate::enums::ButtonId;
use crate::events::{AddTextButtonEvent, ButtonClickedEvent};
use crate::fonts::DefaultFont;
use crate::scenes::{Container, Scene};

pub struct DevCardScene;

#[derive(Component)]
pub struct PlayingCardSprite;

#[derive(Component)]
pub struct PlayingCardText;

pub struct DrawCardEvent(PlayingCard);

impl Scene for DevCardScene {
	fn state(&self) -> AppState {
		AppState::DevCard
	}

	fn add(&self, app: &mut App) {
		app.add_event::<DrawCardEvent>();
	}

	fn on_enter(&self, system_set: SystemSet) -> SystemSet {
		system_set
			.with_system(Self::show_example_card)
	}

	fn on_update(&self, system_set: SystemSet) -> SystemSet {
		system_set
			.with_system(Self::update_card_texture)
			.with_system(Self::update_draw_card_button)
	}

	fn on_exit(&self, system_set: SystemSet) -> SystemSet {
		system_set.with_system(super::cleanup)
	}
}

impl DevCardScene {
	pub fn show_example_card(
		mut commands: Commands,
		//card_texture: Res<PlayingCardTexture>,
		default_font: Res<DefaultFont>,
		mut deck: ResMut<Deck>,
		mut draw_card_event: EventWriter<DrawCardEvent>,
		mut add_button_event: EventWriter<AddTextButtonEvent>,
	) {
		//let mut sprite = card_texture.joker();
		let position = Vec3::new(0.0, 120.0, 0.0);
		//sprite.transform.translation = position;

		//let sprite = commands.spawn((sprite, PlayingCardSprite)).id();

		let text = commands.spawn((
			Text2dBundle {
				text: Text::from_section(String::new(), default_font.style())
					.with_alignment(TextAlignment::CENTER),
				transform: Transform::from_xyz(0.0, position.y - 260.0, 0.0),
				..default()
			},
			PlayingCardText,
		)).id();

		let container = commands.spawn(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
				position: UiRect::top(Val::Percent(50.0)),
				padding: UiRect::all(Val::Px(20.0)),
				align_items: AlignItems::Center,
				justify_content: JustifyContent::FlexEnd,
				flex_direction: FlexDirection::Column,
				..default()
			},
			..default()
		}).id();

		commands.insert_resource(Container(vec![
			container,
			//sprite,
			text,
		]));

		add_button_event.send(AddTextButtonEvent {
			id: ButtonId::DrawCard,
			parent: container,
			size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
			text: vec![String::from("Draw card "), String::from("1")],
			margin: UiRect::default(),
		});

		add_button_event.send(AddTextButtonEvent {
			id: ButtonId::GoToDevMenu,
			parent: container,
			size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
			text: vec![String::from("Back")],
			margin: UiRect::top(Val::Px(20.0)),
		});

		deck.reset();
		deck.shuffle();

		draw_card_event.send(DrawCardEvent(deck.draw().unwrap()));
	}

	pub fn update_card_texture(
		//mut card_sprites: Query<&mut TextureAtlasSprite, With<PlayingCardSprite>>,
		mut texts: Query<&mut Text, With<PlayingCardText>>,
		mut draw_card_event: EventReader<DrawCardEvent>,
	) {
		if let Some(event) = draw_card_event.iter().last() {
			texts.single_mut().sections[0].value = event.0.to_string();
			//card_sprites.single_mut().index = event.0.sprite_index();
		}
	}

	pub fn update_draw_card_button(
		mut state: ResMut<State<AppState>>,
		mut button_clicked_event: EventReader<ButtonClickedEvent>,
		children: Query<&Children>,
		mut texts: Query<&mut Text>,
		mut deck: ResMut<Deck>,
		mut draw_card_event: EventWriter<DrawCardEvent>,
	) {
		for event in button_clicked_event.iter() {
			match event.button_id {
				ButtonId::DrawCard => {
					if let Ok(children) = children.get(event.entity_id) {
						let mut text = texts.get_mut(children[0]).unwrap();
						if let Some(card) = deck.draw() {
							text.sections[1].value = (Deck::MAX - deck.len()).to_string();
							draw_card_event.send(DrawCardEvent(card));
						} else {
							text.sections[0].value = String::from("Deck empty");
							text.sections[1].value = String::new();
						}
					}
				},
				ButtonId::GoToDevMenu => state.set(AppState::Ready).unwrap(),
				_ => {},
			}
		}
	}
}