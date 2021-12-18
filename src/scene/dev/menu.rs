use macroquad::prelude::*;
use macroquad::ui::hash;

impl super::DevMenu {
	pub fn new(assets: std::rc::Rc<crate::assets::Assets>) -> Self {
		Self {
			assets: assets.clone(),
			title: crate::widget::Title::new("Developer Menu", assets),
			window_size: glam::Vec2::ZERO,
		}
	}
}

impl crate::scene::Scene for super::DevMenu {
	fn update(&mut self) -> crate::scene::Command {
		let window_position = vec2(crate::style::WINDOW_PADDING, self.title.safe_y());
		self.window_size = vec2(
			screen_width() - crate::style::WINDOW_PADDING * 2_f32,
			screen_height() - crate::style::WINDOW_PADDING - window_position.y,
		);

		let mut command = crate::scene::Command::None;

		macroquad::ui::widgets::Window::new(hash!(), window_position, self.window_size)
			.titlebar(false)
			.movable(false)
			.ui(&mut *macroquad::ui::root_ui(), |ui| {
				if self.button("Deck", ui) {
					let new_scene = crate::scene::dev::DevDeck::new(self.assets.clone());
					command = crate::scene::Command::PushScene(Box::new(new_scene));
					return;
				}

				if self.button("Scroll", ui) {
					let new_scene = crate::scene::dev::DevScroll::new(self.assets.clone());
					command = crate::scene::Command::PushScene(Box::new(new_scene));
					return;
				}

				if self.button("Input", ui) {
					let new_scene = crate::scene::dev::DevInput::new(self.assets.clone());
					command = crate::scene::Command::PushScene(Box::new(new_scene));
					return;
				}

				if self.button("Storage", ui) {
					let new_scene = crate::scene::dev::DevStorage::new(self.assets.clone());
					command = crate::scene::Command::PushScene(Box::new(new_scene));
					return;
				}

				if self.button("Back", ui) {
					command = crate::scene::Command::PopScene;
					return;
				}
			});

		self.title.ui(&mut *macroquad::ui::root_ui());

		command
	}
}

impl super::DevMenu {
	fn button(&self, label: &str, ui: &mut macroquad::ui::Ui) -> bool {
		macroquad::ui::widgets::Button::new(label)
			.size(vec2(
				self.window_size.x - crate::style::WINDOW_PADDING * 2_f32 - 4_f32,
				96_f32,
			))
			.ui(ui)
	}
}