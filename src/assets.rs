use macroquad::prelude::*;

/// Available texture
#[derive(Eq, PartialEq, Hash, strum_macros::Display, strum_macros::EnumIter)]
pub enum AssetTexture {
	#[strum(serialize = "playing_cards")]
	PlayingCards,
	#[strum(serialize = "ui")]
	Ui,
}

/// Available font
#[derive(Eq, PartialEq, Hash, strum_macros::Display, strum_macros::EnumIter)]
pub enum AssetFont {
	#[strum(serialize = "bold")]
	Bold,
	#[strum(serialize = "mini")]
	Mini,
}

/// Available image
#[derive(Eq, PartialEq, Hash, strum_macros::Display, strum_macros::EnumIter)]
pub enum AssetImage {
	#[strum(serialize = "panel")]
	Panel,
}

/// Container for all loaded assets
pub struct Assets {
	/// Textures loaded as Texture2D (GPU)
	textures: std::collections::HashMap<AssetTexture, Texture2D>,
	/// Fonts loaded as a byte array
	/// (UI loads fonts as bytes directly)
	fonts: std::collections::HashMap<AssetFont, Vec<u8>>,
	/// Images loaded as Image (CPU)
	images: std::collections::HashMap<AssetImage, Image>,
}

impl Assets {
	/// Instance a new asset loader,
	/// doesn't actually load anything
	pub fn new() -> Self {
		Self {
			textures: std::collections::HashMap::new(),
			fonts: std::collections::HashMap::new(),
			images: std::collections::HashMap::new(),
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

	/// Load all images
	async fn load_images(&mut self) {
		use strum::IntoEnumIterator;
		for image in AssetImage::iter() {
			let path = format!("image/{}.png", image);
			match load_image(&path).await {
				Ok(i) => self.images.insert(image, i),
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
		self.load_images().await;
	}

	/// Get a loaded texture
	pub fn texture(&self, name: &AssetTexture) -> Texture2D {
		self.textures.get(name).unwrap().clone()
	}

	/// Get a loaded texture from a tileset
	/// (assumes square tiles)
	pub fn tileset_texture(&self, name: &AssetTexture, index: u16) -> Texture2D {
		let image = self.texture(name).get_texture_data();
		let source = Rect {
			x: (image.width * index) as f32,
			y: 0_f32,
			w: image.width as f32,
			h: image.height as f32,
		};

		Texture2D::from_image(&image.sub_image(source))
	}

	/// Get font from data and parse
	pub fn font(&self, name: &AssetFont) -> Font {
		load_ttf_font_from_bytes(self.font_data(name)).unwrap()
	}

	/// Get byte data for font
	pub fn font_data(&self, name: &AssetFont) -> &[u8] {
		self.fonts.get(name).unwrap()
	}

	/// Get a loaded image
	pub fn image(&self, name: &AssetImage) -> Image {
		self.images.get(name).unwrap().clone()
	}
}
