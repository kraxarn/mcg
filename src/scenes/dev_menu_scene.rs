use bevy::prelude::*;
use crate::AppState;
use crate::events::{AddTextButtonEvent, ButtonClickedEvent};
use crate::scenes::Scene;

const DEV_CARD_ID: &str = "dev_card";
const BLACK_JACK_ID: &str = "black_jack";

pub struct DevMenuScene;

#[derive(Resource)]
pub struct Container(pub Entity);

impl Scene for DevMenuScene {
	fn state(&self) -> AppState {
		AppState::Ready
	}

	fn add(&self, _: &mut App) {}

	fn on_enter(&self, system_set: SystemSet) -> SystemSet {
		system_set
			.with_system(Self::show_buttons)
	}

	fn on_update(&self, system_set: SystemSet) -> SystemSet {
		system_set
			.with_system(Self::update_button_clicked)
	}

	fn on_exit(&self, system_set: SystemSet) -> SystemSet {
		system_set.with_system(Self::cleanup)
	}
}

impl DevMenuScene {
	pub fn show_buttons(
		mut commands: Commands,
		mut add_button_event: EventWriter<AddTextButtonEvent>,
	) {
		let container = commands.spawn(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				position: UiRect::all(Val::Px(0.0)),
				padding: UiRect::all(Val::Px(40.0)),
				align_items: AlignItems::Center,
				justify_content: JustifyContent::FlexStart,
				flex_direction: FlexDirection::Column,
				..default()
			},
			..default()
		}).id();

		commands.insert_resource(Container(container));

		add_button_event.send(AddTextButtonEvent {
			id: String::from(DEV_CARD_ID),
			parent: container,
			size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
			text: vec![String::from("Card viewer")],
			margin: UiRect::default(),
		});

		add_button_event.send(AddTextButtonEvent {
			id: String::from(BLACK_JACK_ID),
			parent: container,
			size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
			text: vec![String::from("Black jack")],
			margin: UiRect::top(Val::Px(40.0)),
		});
	}

	pub fn update_button_clicked(
		mut state: ResMut<State<AppState>>,
		mut clicked: EventReader<ButtonClickedEvent>,
	) {
		for event in clicked.iter() {
			match event.button_id.as_str() {
				DEV_CARD_ID => state.set(AppState::DevCard).unwrap(),
				_ => panic!("Unknown button: {}", &event.button_id),
			}
		}
	}

	pub fn cleanup(mut commands: Commands, container: Res<Container>) {
		commands.entity(container.0).despawn_recursive();
	}
}