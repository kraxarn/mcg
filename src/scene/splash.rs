use macroquad::prelude::*;

pub async fn splash() {
	let splash = load_texture("texture/splash.png").await.unwrap();

	clear_background(crate::color::BACKGROUND);

	draw_texture(
		splash,
		screen_width() / 2_f32 - splash.width() / 2_f32,
		screen_height() / 2_f32 - splash.height() / 2_f32,
		WHITE,
	);

	let loading_text = "Loading assets...";
	let text_size = measure_text(loading_text, None, 32_u16, 1_f32);

	draw_text(
		loading_text,
		screen_width() / 2_f32 - text_size.width / 2_f32,
		screen_height() / 2_f32 + splash.height(),
		32_f32,
		crate::color::FOREGROUND_ALT,
	);

	next_frame().await;
}
