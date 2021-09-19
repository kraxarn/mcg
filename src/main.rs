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

#[macroquad::main(window_conf)]
async fn main() {
	macroquad::file::set_pc_assets_folder("assets");

	let mut game = game::Game::new();
	game.scene_manager
		.push(Box::new(scene::dev::DevDeck::new().await));

	loop {
		clear_background(color::BACKGROUND);
		game.scene_manager.update();
		next_frame().await;
	}
}
