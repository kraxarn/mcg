#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum AppState {
	SetupTextures,
	SetupFonts,
	Ready,
	DevCard,
}