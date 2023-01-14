use bevy::prelude::*;
use crate::entities::PlayingCard;
use crate::textures::PlayingCardTexture;

pub fn card_row(
	parent: &mut ChildBuilder,
	card_texture: &Res<PlayingCardTexture>,
	container_size: Size,
	card_size: Size,
	cards: &Vec<PlayingCard>,
) {
	parent.spawn(NodeBundle {
		style: Style {
			size: container_size,
			justify_content: JustifyContent::SpaceAround,
			flex_direction: FlexDirection::Row,
			..default()
		},
		..default()
	}).with_children(|parent| {
		for card in cards {
			let image = card_texture.card(card);
			parent.spawn(ImageBundle {
				style: Style {
					size: card_size,
					..default()
				},
				image,
				..default()
			});
		}
	});
}