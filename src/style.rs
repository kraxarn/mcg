/// Font size in buttons
pub const BUTTON_FONT_SIZE: u16 = 24_u16;

fn button_style(assets: std::rc::Rc<crate::assets::Assets>) -> macroquad::ui::Style {
	macroquad::ui::root_ui()
		.style_builder()
		.font(assets.font_data(&crate::assets::AssetFont::Bold))
		.unwrap()
		.font_size(BUTTON_FONT_SIZE)
		.text_color(crate::color::FOREGROUND)
		.color(crate::color::BUTTON)
		.color_hovered(crate::color::BUTTON_HOVER)
		.color_clicked(crate::color::BUTTON_CLICK)
		.build()
}

/// Default UI skin for app
pub fn skin(assets: std::rc::Rc<crate::assets::Assets>) -> macroquad::ui::Skin {
	macroquad::ui::Skin {
		button_style: button_style(assets.clone()),
		..macroquad::ui::root_ui().default_skin()
	}
}
