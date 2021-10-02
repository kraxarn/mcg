use macroquad::prelude::*;

mod title;

/// Banner with text
pub struct Title {
	label: String,
	texture: Texture2D,
	skin: Option<macroquad::ui::Skin>,
	font_data: Vec<u8>,
	font_size: u16,
	/// Only used when measuring text size for alignment
	font: macroquad::text::Font,
}
