use bevy::prelude::*;
use crate::entities::PlayingCard;
use crate::fonts::DefaultFont;

pub fn card_text(
	parent: &mut ChildBuilder,
	default_font: &Res<DefaultFont>,
	cards: &Vec<PlayingCard>,
) {
	let sum: u8 = cards.iter()
		.map(|card| card.value as u8 + 1)
		.sum();

	parent.spawn(TextBundle {
		text: Text::from_section(sum.to_string(), default_font.style()),
		style: Style {
			margin: UiRect::all(Val::Px(20.0)),
			..default()
		},
		..default()
	});
}