use macroquad::color::Color;

/// Text color (\#eceff1)
pub const FOREGROUND: Color = Color::new(0.93, 0.94, 0.95, 1.00);

/// Clear color (\#3399da)
pub const BACKGROUND: Color = Color::new(0.2, 0.6, 0.85, 1.00);

/// Button background color (\#a3703a)
pub const BUTTON: Color = Color::new(0.64, 0.44, 0.34, 1.00);
pub const BUTTON_HOVER: Color = Color::new(BUTTON.r * 0.9, BUTTON.g * 0.9, BUTTON.b * 0.9, 1.00);
pub const BUTTON_CLICK: Color = Color::new(BUTTON.r * 0.8, BUTTON.g * 0.8, BUTTON.b * 0.8, 1.00);
