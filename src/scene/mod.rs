pub mod dev;

/// A scene, or screen, in the game
pub trait Scene {
	/// Update logic and draw
	fn update(&mut self);
}
