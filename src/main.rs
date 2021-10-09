use macroquad::prelude::*;

mod assets;
mod color;
mod entity;
mod game;
mod scene;
mod scene_manager;
mod settings;
mod style;
mod tileset;
mod time;
mod widget;

pub const APP_NAME: &str = "Card Games: Alpha";

fn window_conf() -> Conf {
	Conf {
		window_title: format!("{} {}", APP_NAME, env!("CARGO_PKG_VERSION")),
		window_width: 540_i32,
		window_height: 960_i32,
		window_resizable: true,
		..Default::default()
	}
}

#[macroquad::main(window_conf)]
async fn main() {
	macroquad::file::set_pc_assets_folder("assets");
	rand::srand(time::current_timestamp());

	let mut assets = assets::Assets::new();
	scene::splash::splash().await;
	assets.load_all().await;

	let pattern = assets.texture(&crate::assets::AssetTexture::Pattern);

	let mut game = game::Game::new(assets);
	game.scene_manager
		.push(Box::new(scene::dev::DevMenu::new(game.assets())));

	loop {
		clear_background(color::BACKGROUND);

		for x in 0..(screen_width() / pattern.width()).ceil() as u32 {
			for y in 0..(screen_height() / pattern.height()).ceil() as u32 {
				draw_texture(
					pattern,
					x as f32 * pattern.width(),
					y as f32 * pattern.height(),
					WHITE,
				);
			}
		}

		game.push_skin();
		game.scene_manager.update();

		// Draw FPS in debug
		#[cfg(debug_assertions)]
		draw_text(&get_fps().to_string(), 16_f32, 32_f32, 32_f32, WHITE);

		next_frame().await;
	}
}
