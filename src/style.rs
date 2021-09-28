use macroquad::prelude::*;

/// Font size in buttons
pub const BUTTON_FONT_SIZE: u16 = 24_u16;

fn button_style(assets: std::rc::Rc<crate::assets::Assets>) -> macroquad::ui::Style {
	const PADDING: f32 = 10_f32;

	macroquad::ui::root_ui()
		.style_builder()
		.font(assets.font_data(&crate::assets::AssetFont::Bold))
		.unwrap()
		.font_size(BUTTON_FONT_SIZE)
		.text_color(crate::color::FOREGROUND)
		.background(assets.image(&crate::assets::AssetImage::Button))
		.background_clicked(assets.image(&crate::assets::AssetImage::ButtonClicked))
		.background_margin(RectOffset::new(PADDING, PADDING, PADDING, PADDING))
		.build()
}

fn window_style(assets: std::rc::Rc<crate::assets::Assets>) -> macroquad::ui::Style {
	const PADDING: f32 = 8_f32;

	macroquad::ui::root_ui()
		.style_builder()
		.font(assets.font_data(&crate::assets::AssetFont::Thick))
		.unwrap()
		.background(assets.image(&crate::assets::AssetImage::Panel))
		.background_margin(RectOffset::new(PADDING, PADDING, PADDING, PADDING))
		.margin(RectOffset::new(-PADDING, -PADDING, -PADDING, -PADDING))
		.build()
}

/// Default UI skin for app
pub fn skin(assets: std::rc::Rc<crate::assets::Assets>) -> macroquad::ui::Skin {
	macroquad::ui::Skin {
		// label_style
		button_style: button_style(assets.clone()),
		// tabbar_style
		window_style: window_style(assets.clone()),
		// editbox_style
		// window_titlebar_style
		// scrollbar_style
		// scrollbar_handle_style
		// checkbox_style
		// group_style
		..macroquad::ui::root_ui().default_skin()
	}
}
