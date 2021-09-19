use macroquad::prelude::*;
use macroquad::ui::hash;

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

async fn draw_button(deck: &mut entity::Deck, card: &mut entity::PlayingCard) {
	let window_size = vec2(screen_width() - 64_f32, 96_f32);
	let window_position = vec2(32_f32, screen_height() - window_size.y - 64_f32);

	macroquad::ui::widgets::Window::new(hash!(), window_position, window_size)
		.titlebar(false)
		.ui(&mut *macroquad::ui::root_ui(), |ui| {
			let button = macroquad::ui::widgets::Button::new(if deck.len() > 0 {
				"Draw card"
			} else {
				"Empty deck"
			})
			.position(glam::Vec2::ZERO)
			.size(window_size);

			if button.ui(ui) {
				match deck.draw() {
					Some(c) => {
						*card = c;
					}
					None => {}
				};
			}
		});
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

	let mut deck = entity::Deck::new().await;
	deck.shuffle();

	let mut current_card = deck.draw().unwrap();
	let mut card_size = deck.card_size();
	let card_width = card_size.x;
	let card_height = card_size.y;

	let button_style = macroquad::ui::root_ui()
		.style_builder()
		.font(load_file("font/bold.ttf").await.unwrap().as_slice())
		.unwrap()
		.font_size(bold_font.font_size)
		.text_color(color::FOREGROUND)
		.color(color::BUTTON)
		.color_hovered(color::BUTTON_HOVER)
		.color_clicked(color::BUTTON_CLICK)
		.build();

	let skin = macroquad::ui::Skin {
		button_style,
		..macroquad::ui::root_ui().default_skin()
	};

	loop {
		let window_width = screen_width();
		let window_height = screen_height();

		clear_background(color::BACKGROUND);

		// Draw FPS
		draw_text_ex(&get_fps().to_string(), 16_f32, 32_f32, mini_font);

		// Card texture scale (40% of screen width)
		let card_scale = (window_width * 0.4) / card_width;

		// Draw the card itself
		let card_y = (window_height / 2_f32) - (card_height * card_scale / 2_f32);
		draw_texture_ex(
			deck.card_atlas(),
			(window_width / 2_f32) - (card_width * card_scale / 2_f32),
			card_y,
			WHITE,
			DrawTextureParams {
				dest_size: Some(card_size * card_scale),
				..deck.card_source(&current_card)
			},
		);

		// Measure card name size to center text
		let card_name = current_card.to_string();
		let card_name_width = measure_text(
			&card_name,
			Some(bold_font.font),
			bold_font.font_size as u16,
			bold_font.font_scale,
		)
		.width;

		// Draw name of card centered under card
		draw_text_ex(
			&card_name,
			(window_width / 2_f32) - (card_name_width / 2_f32),
			card_y + card_height * card_scale + 64_f32,
			bold_font,
		);

		macroquad::ui::root_ui().push_skin(&skin);
		draw_button(&mut deck, &mut current_card).await;

		next_frame().await;
	}
}
