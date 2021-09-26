use crate::scene::Scene;
use std::cell::RefMut;

/// Basic scene manager
pub struct SceneManager {
	scenes: std::rc::Rc<std::cell::RefCell<Vec<Box<dyn crate::scene::Scene>>>>,
}

impl SceneManager {
	/// Initialize, initially empty
	pub fn new() -> Self {
		Self {
			scenes: std::rc::Rc::new(std::cell::RefCell::new(Vec::new())),
		}
	}

	/// Mutable reference to scenes
	fn scenes_mut(&self) -> RefMut<'_, Vec<Box<dyn Scene>>> {
		self.scenes.borrow_mut()
	}

	/// Update current scene
	pub fn update(&self) {
		match self.scenes_mut().last_mut() {
			Some(s) => s.update(),
			None => {}
		};
	}

	/// Add a new scene
	pub fn push(&self, scene: Box<dyn crate::scene::Scene>) {
		self.scenes_mut().push(scene);
	}

	/// Pop current scene, going back to the previous one
	/// (exits if last scene)
	pub fn pop(&self) {
		self.scenes_mut().pop();

		if self.scenes_mut().is_empty() {
			std::process::exit(0);
		}
	}
}
