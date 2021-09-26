pub mod dev;

/// Command to execute after frame update
#[derive(strum_macros::Display)]
pub enum Command {
	/// Continue running
	None,
	/// Go back to previous scene
	PopScene,
	/// Go to a new scene
	PushScene(Box<dyn crate::scene::Scene>),
}

/// A scene, or screen, in the game
pub trait Scene {
	/// Update logic and draw
	fn update(&mut self) -> Command;
}
