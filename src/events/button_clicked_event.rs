use bevy::prelude::*;
use crate::colors;
use crate::events::ButtonId;

pub struct ButtonClickedEvent {
	pub button_id: String,
	pub entity_id: Entity,
}

pub fn update_button_clicked(
	mut interactions: Query<(&Interaction, &mut BackgroundColor, Entity, &ButtonId),
		(Changed<Interaction>, With<Button>)>,
	mut clicked_event: EventWriter<ButtonClickedEvent>,
) {
	for (interaction, mut color, entity, button_id) in &mut interactions {
		let new_color = match *interaction {
			Interaction::Clicked => colors::BUTTON_CLICKED,
			Interaction::Hovered => colors::BUTTON_HOVERED,
			Interaction::None => colors::BUTTON,
		};

		if color.0 == colors::BUTTON_CLICKED {
			clicked_event.send(ButtonClickedEvent {
				button_id: button_id.0.clone(),
				entity_id: entity,
			});
		}

		*color = BackgroundColor::from(new_color);
	}
}