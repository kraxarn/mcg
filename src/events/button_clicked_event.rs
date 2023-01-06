use bevy::prelude::*;
use crate::colors;

pub struct ButtonClickedEvent(pub Entity);

pub fn update_button_clicked(
	mut interactions: Query<(&Interaction, &mut BackgroundColor, Entity),
		(Changed<Interaction>, With<Button>)>,
	mut clicked_event: EventWriter<ButtonClickedEvent>,
) {
	for (interaction, mut color, entity) in &mut interactions {
		let new_color = match *interaction {
			Interaction::Clicked => colors::BUTTON_CLICKED,
			Interaction::Hovered => colors::BUTTON_HOVERED,
			Interaction::None => colors::BUTTON,
		};

		if color.0 == colors::BUTTON_CLICKED {
			clicked_event.send(ButtonClickedEvent(entity));
		}

		*color = BackgroundColor::from(new_color);
	}
}