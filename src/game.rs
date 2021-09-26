pub struct Game {
	pub assets: std::rc::Rc<crate::assets::Assets>,
	pub scene_manager: crate::scene_manager::SceneManager,
}

impl Game {
	pub fn new(assets: crate::assets::Assets) -> Self {
		Self {
			scene_manager: crate::scene_manager::SceneManager::new(),
			assets: std::rc::Rc::new(assets),
		}
	}

	pub fn assets(&self) -> std::rc::Rc<crate::assets::Assets> {
		self.assets.clone()
	}
}
