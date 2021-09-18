use macroquad::prelude::*;

mod color;
mod entity;

pub const APP_NAME: &str = "MobileCardGames: Alpha";

fn window_conf() -> Conf {
	Conf {
		window_title: APP_NAME.to_owned(),
		window_width: 540_i32,
		window_height: 960_i32,
		window_resizable: true,
		..Default::default()
	}
}

#[macroquad::main(window_conf)]
async fn main() {
	macroquad::file::set_pc_assets_folder("assets");

	let mini_font = TextParams {
		font: macroquad::text::load_ttf_font("font/mini.ttf")
			.await
			.unwrap(),
		font_size: 24_u16,
		color: color::FOREGROUND,
		..Default::default()
	};

	let bold_font = TextParams {
		font: macroquad::text::load_ttf_font("font/bold.ttf")
			.await
			.unwrap(),
		font_size: 28_u16,
		color: color::FOREGROUND,
		..Default::default()
	};

	let mut deck = entity::playing_card::deck();
	entity::playing_card::shuffle(&mut deck);

	let first_card = &mut deck[0];
	first_card.load_texture().await;

	let card_width = first_card.texture().width();
	let card_height = first_card.texture().height();

	// Measure card name size to center text
	let card_name = first_card.to_string();
	let card_name_width = measure_text(
		&card_name,
		Some(bold_font.font),
		bold_font.font_size as u16,
		bold_font.font_scale,
	)
	.width;

	loop {
		let window_width = screen_width();
		let window_height = screen_height();

		clear_background(color::BACKGROUND);

		// Draw FPS
		draw_text_ex(&get_fps().to_string(), 16_f32, 32_f32, mini_font);

		// Draw the card itself
		let card_y = (window_height / 2_f32) - (card_height / 2_f32);
		draw_texture(
			first_card.texture(),
			(window_width / 2_f32) - (card_width / 2_f32),
			card_y,
			WHITE,
		);

		// Draw name of card centered under card
		draw_text_ex(
			&card_name,
			(window_width / 2_f32) - (card_name_width / 2_f32),
			card_y + card_height + 64_f32,
			bold_font,
		);

		next_frame().await;
	}
}
