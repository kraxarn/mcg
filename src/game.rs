pub struct Game {
	pub scene_manager: crate::scene_manager::SceneManager,
}

impl Game {
	pub fn new() -> Self {
		Self {
			scene_manager: crate::scene_manager::SceneManager::new(),
		}
	}
}
