use bevy::prelude::Color;

/// Primary text color for labels etc.
pub const FOREGROUND: Color = Color::rgb(0.93, 0.94, 0.95);

/// Clear color
pub const BACKGROUND: Color = Color::rgb(0.22, 0.28, 0.31);

/// Background color for buttons
pub const BUTTON: Color = Color::rgb(0.27, 0.35, 0.39);

/// Background color for buttons on hover
pub const BUTTON_HOVERED: Color = BUTTON;

/// Background color for buttons on press
pub const BUTTON_CLICKED: Color = Color::rgb(0.33, 0.43, 0.48);