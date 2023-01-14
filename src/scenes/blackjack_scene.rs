use bevy::prelude::*;
use crate::AppState;
use crate::entities::{Deck, PlayingCard};
use crate::enums::ButtonId;
use crate::events::AddTextButtonEvent;
use crate::fonts::DefaultFont;
use crate::scenes::Scene;
use crate::textures::PlayingCardTexture;

pub struct BlackjackScene;

#[derive(Component)]
struct PlayingCardSprite;

impl Scene for BlackjackScene {
	fn state(&self) -> AppState {
		AppState::Blackjack
	}

	fn add(&self, _: &mut App) {}

	fn on_enter(&self, system_set: SystemSet) -> SystemSet {
		system_set.with_system(Self::load_ui)
	}

	fn on_update(&self, system_set: SystemSet) -> SystemSet {
		system_set
	}

	fn on_exit(&self, system_set: SystemSet) -> SystemSet {
		system_set.with_system(super::cleanup)
	}
}

impl BlackjackScene {
	pub fn load_ui(
		mut commands: Commands,
		card_texture: Res<PlayingCardTexture>,
		mut deck: ResMut<Deck>,
		default_font: Res<DefaultFont>,
		mut add_button_event: EventWriter<AddTextButtonEvent>,
	) {
		deck.reset();
		deck.shuffle();

		let dealer_cards: Vec<PlayingCard> = (0..2)
			.map(|_| deck.draw().unwrap())
			.collect();

		let player_cards: Vec<PlayingCard> = (0..2)
			.map(|_| deck.draw().unwrap())
			.collect();

		commands.spawn(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				padding: UiRect::all(Val::Px(40.0)),
				align_items: AlignItems::Center,
				flex_direction: FlexDirection::Column,
				..default()
			},
			..default()
		}).with_children(|parent| {
			//region Dealer cards
			crate::bundles::card_row(parent, &card_texture,
				Size::new(Val::Percent(100.0), Val::Px(175.0)),
				Size::new(Val::Auto, Val::Percent(100.0)),
				&dealer_cards,
			);
			//endregion
			//region Dealer text
			crate::bundles::card_text(parent, &default_font, &dealer_cards);
			//endregion
			//region Spacer
			parent.spawn(NodeBundle {
				style: Style {
					flex_grow: 1.0,
					..default()
				},
				..default()
			});
			//endregion
			//region Player text
			crate::bundles::card_text(parent, &default_font, &player_cards);
			//endregion
			//region Player cards
			crate::bundles::card_row(parent, &card_texture,
				Size::new(Val::Percent(100.0), Val::Px(200.0)),
				Size::new(Val::Auto, Val::Percent(100.0)),
				&player_cards);
			//endregion
			//region Buttons
			let buttons = parent.spawn(NodeBundle {
				style: Style {
					size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
					margin: UiRect::top(Val::Px(40.0)),
					flex_direction: FlexDirection::Row,
					..default()
				},
				..default()
			}).id();

			add_button_event.send(AddTextButtonEvent {
				id: ButtonId::BlackjackHit,
				parent: buttons,
				size: Size::new(Val::Percent(50.0), Val::Px(100.0)),
				text: vec![String::from("Hit")],
				margin: UiRect::right(Val::Px(10.0)),
			});

			add_button_event.send(AddTextButtonEvent {
				id: ButtonId::BlackjackStand,
				parent: buttons,
				size: Size::new(Val::Percent(50.0), Val::Px(100.0)),
				text: vec![String::from("Stand")],
				margin: UiRect::left(Val::Px(10.0)),
			})
			//endregion
		});
	}
}