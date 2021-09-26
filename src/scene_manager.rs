/// Basic scene manager
pub struct SceneManager {
	scenes: Vec<Box<dyn crate::scene::Scene>>,
}

impl SceneManager {
	/// Initialize, initially empty
	pub fn new() -> Self {
		Self { scenes: Vec::new() }
	}

	/// Update current scene
	pub fn update(&mut self) {
		let command = match self.scenes.last_mut() {
			Some(s) => s.update(),
			None => crate::scene::Command::None,
		};

		match command {
			crate::scene::Command::PopScene => {
				self.pop();
				()
			}
			crate::scene::Command::PushScene(s) => self.push(s),
			crate::scene::Command::None => {}
		};
	}

	/// Add a new scene
	pub fn push(&mut self, scene: Box<dyn crate::scene::Scene>) {
		self.scenes.push(scene);
	}

	/// Pop current scene, going back to the previous one
	/// (exits if last scene)
	pub fn pop(&mut self) {
		self.scenes.pop();

		if self.scenes.is_empty() {
			std::process::exit(0);
		}
	}
}
