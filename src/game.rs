pub struct Game {
	pub scene_manager: crate::scene_manager::SceneManager,
}

impl Game {
	pub fn new() -> std::rc::Rc<Self> {
		std::rc::Rc::new(Self {
			scene_manager: crate::scene_manager::SceneManager::new(),
		})
	}
}
