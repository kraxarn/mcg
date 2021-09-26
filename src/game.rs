pub struct Game {
	assets: std::rc::Rc<crate::assets::Assets>,
	skin: macroquad::ui::Skin,
	pub scene_manager: crate::scene_manager::SceneManager,
}

impl Game {
	pub fn new(assets: crate::assets::Assets) -> Self {
		let assets_ref = std::rc::Rc::new(assets);
		Self {
			scene_manager: crate::scene_manager::SceneManager::new(),
			assets: assets_ref.clone(),
			skin: crate::style::skin(assets_ref.clone()),
		}
	}

	/// Get a new reference to asset loader
	pub fn assets(&self) -> std::rc::Rc<crate::assets::Assets> {
		self.assets.clone()
	}

	/// Set default skin as skin for current frame
	pub fn push_skin(&self) {
		macroquad::ui::root_ui().push_skin(&self.skin);
	}
}
