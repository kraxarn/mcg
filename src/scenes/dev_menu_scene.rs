use bevy::prelude::*;
use crate::AppState;
use crate::enums::ButtonId;
use crate::events::{AddTextButtonEvent, ButtonClickedEvent};
use crate::scenes::{Container, Scene};

pub struct DevMenuScene;

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
		system_set.with_system(super::cleanup)
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

		commands.insert_resource(Container(vec![container]));

		add_button_event.send(AddTextButtonEvent {
			id: ButtonId::GoToDevCard,
			parent: container,
			size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
			text: vec![String::from("Card viewer")],
			margin: UiRect::default(),
		});

		add_button_event.send(AddTextButtonEvent {
			id: ButtonId::GoToBlackJack,
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
			match event.button_id {
				ButtonId::GoToDevCard => state.set(AppState::DevCard).unwrap(),
				_ => {},
			}
		}
	}
}