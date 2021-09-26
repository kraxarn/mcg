use macroquad::prelude::*;

mod color;
mod entity;
mod game;
mod scene;
mod scene_manager;

pub const APP_NAME: &str = "Card Games: Alpha";

fn window_conf() -> Conf {
	Conf {
		window_title: APP_NAME.to_owned(),
		window_width: 540_i32,
		window_height: 960_i32,
		window_resizable: true,
		..Default::default()
	}
}

async fn splash() {
	let splash = load_texture("texture/splash.png").await.unwrap();

	clear_background(color::BACKGROUND);

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
		color::FOREGROUND,
	);

	next_frame().await;
}

#[macroquad::main(window_conf)]
async fn main() {
	macroquad::file::set_pc_assets_folder("assets");

	splash().await;

	let mut game = game::Game::new();
	game.scene_manager
		.push(Box::new(scene::dev::DevDeck::new().await));

	loop {
		clear_background(color::BACKGROUND);
		game.scene_manager.update();
		next_frame().await;
	}
}
