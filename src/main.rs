use macroquad::prelude::*;

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

	let mut max_text_width = 0_f32;

	let text_params = TextParams {
		font: macroquad::text::load_ttf_font("font/mini.ttf")
			.await
			.unwrap(),
		font_size: 32_u16,
		color: WHITE,
		..Default::default()
	};

	loop {
		let text = format!("FPS: {}", get_fps());

		let window_width = macroquad::window::screen_width();
		let text_width = measure_text(
			&text,
			Some(text_params.font),
			text_params.font_size as u16,
			text_params.font_scale,
		)
		.width;

		if text_width > max_text_width {
			max_text_width = text_width;
		}

		clear_background(BLACK);

		draw_text_ex(
			&text,
			(window_width / 2_f32) - (max_text_width / 2_f32),
			64_f32,
			text_params,
		);

		next_frame().await;
	}
}
