use macroquad::prelude::*;

impl super::Title {
	pub fn new(label: &str, assets: std::rc::Rc<crate::assets::Assets>) -> Self {
		let ui = &mut *macroquad::ui::root_ui();
		let mut title = Self {
			label: String::new(),
			label_dimensions: TextDimensions {
				width: 0_f32,
				height: 0_f32,
				offset_y: 0_f32,
			},
			texture: assets.texture(&crate::assets::AssetTexture::Banner),
			font_data: Vec::from(assets.font_data(&crate::assets::AssetFont::Thick)),
			font: assets.font(&crate::assets::AssetFont::Thick),
			skin: ui.default_skin(),
			font_size: 18_u16,
		};

		title.set_label(label);
		title.skin = title.skin(ui);

		title
	}

	fn set_label(&mut self, label: &str) {
		self.label = label.to_owned();
		self.label_dimensions = measure_text(&self.label, Some(self.font), self.font_size, 1_f32);
	}

	/// Safe position to draw stuff under title
	pub fn safe_y(&self) -> f32 {
		self.position().y + self.texture.height() + 8_f32
	}

	fn skin(&self, ui: &mut macroquad::ui::Ui) -> macroquad::ui::Skin {
		macroquad::ui::Skin {
			label_style: ui
				.style_builder()
				.font(&self.font_data)
				.unwrap()
				.font_size(self.font_size)
				.text_color(crate::color::FOREGROUND_ALT)
				.build(),
			..ui.default_skin()
		}
	}

	fn position(&self) -> glam::Vec2 {
		glam::Vec2::new(screen_width() / 2_f32 - self.texture.width() / 2_f32, 4_f32)
	}

	pub fn ui(&mut self, ui: &mut macroquad::ui::Ui) {
		ui.push_skin(&self.skin);

		let texture_position = self.position();
		let label_position = glam::Vec2::new(
			texture_position.x + self.texture.width() / 2_f32 - self.label_dimensions.width / 2_f32,
			texture_position.y + self.texture.height() / 2_f32
				- self.label_dimensions.height / 2_f32,
		);

		macroquad::ui::widgets::Texture::new(self.texture)
			.size(self.texture.width(), self.texture.height())
			.position(texture_position)
			.ui(ui);

		macroquad::ui::widgets::Label::new(&self.label)
			.position(label_position)
			.ui(ui);

		ui.pop_skin();
	}
}
