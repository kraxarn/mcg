use macroquad::prelude::*;

/// Available texture
#[derive(Eq, PartialEq, Hash, strum_macros::Display, strum_macros::EnumIter)]
pub enum AssetTexture {
	/// texture/playing_cards.png
	#[strum(serialize = "playing_cards")]
	PlayingCards,
}

/// Available font
#[derive(Eq, PartialEq, Hash, strum_macros::Display, strum_macros::EnumIter)]
pub enum AssetFont {
	/// font/bold.ttf
	#[strum(serialize = "bold")]
	Bold,
	/// font/mini.ttf
	#[strum(serialize = "mini")]
	Mini,
}

/// Container for all loaded assets
pub struct Assets {
	/// Textures loaded as Texture2D
	textures: std::collections::HashMap<AssetTexture, Texture2D>,
	/// Fonts loaded as a byte array
	/// (UI loads fonts as bytes directly)
	fonts: std::collections::HashMap<AssetFont, Vec<u8>>,
}

impl Assets {
	/// Instance a new asset loader,
	/// doesn't actually load anything
	pub fn new() -> Self {
		Self {
			textures: std::collections::HashMap::new(),
			fonts: std::collections::HashMap::new(),
		}
	}

	/// Load all textures
	async fn load_textures(&mut self) {
		use strum::IntoEnumIterator;
		for texture in AssetTexture::iter() {
			let path = format!("texture/{}.png", texture);
			match load_texture(&path).await {
				Ok(t) => self.textures.insert(texture, t),
				Err(e) => {
					error!("Failed to load \"{}\": {}", path, e);
					None
				}
			};
		}
	}

	/// Load all fonts
	async fn load_fonts(&mut self) {
		use strum::IntoEnumIterator;
		for font in AssetFont::iter() {
			let path = format!("font/{}.ttf", font);
			match load_file(&path).await {
				Ok(v) => self.fonts.insert(font, v),
				Err(e) => {
					error!("Failed to load \"{}\": {}", path, e);
					None
				}
			};
		}
	}

	/// Load all assets
	pub async fn load_all(&mut self) {
		self.load_textures().await;
		self.load_fonts().await;
	}

	/// Get a loaded texture
	pub fn texture(&self, name: &AssetTexture) -> Texture2D {
		self.textures.get(name).unwrap().clone()
	}

	/// Get font from data and parse
	pub fn font(&self, name: &AssetFont) -> Font {
		load_ttf_font_from_bytes(self.font_data(name)).unwrap()
	}

	/// Get byte data for font
	pub fn font_data(&self, name: &AssetFont) -> &[u8] {
		self.fonts.get(name).unwrap()
	}
}
