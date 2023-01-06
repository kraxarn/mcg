use bevy::prelude::*;
use bevy::ui::Size;
use crate::colors;
use crate::fonts::DefaultFont;

pub struct AddTextButtonEvent {
	pub parent: Entity,
	pub size: Size,
	pub text: Vec<String>,
}

pub fn update_add_text_button(
	mut commands: Commands,
	default_font: Res<DefaultFont>,
	mut events: EventReader<AddTextButtonEvent>,
) {
	for event in events.iter() {
		if let Some(mut commands) = commands.get_entity(event.parent) {
			commands.add_children(|parent| {
				parent.spawn(ButtonBundle {
					style: Style {
						size: event.size,
						align_items: AlignItems::Center,
						justify_content: JustifyContent::Center,
						..default()
					},
					background_color: colors::BUTTON.into(),
					..default()
				}).with_children(|parent| {
					parent.spawn(TextBundle::from_sections(event.text.iter()
						.map(|text| TextSection::new(text, default_font.style()))));
				});
			});
		}
	}
}