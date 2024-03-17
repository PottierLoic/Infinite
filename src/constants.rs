use crate::*;

// Sizes
pub const BORDER_SIZE: f32 = 50.0;
pub const BOARD_SIZE: f32 = 400.0;
pub const GRID_SIZE: f32 = 20.0;
pub const SCREEN_SIZE: f32 = BORDER_SIZE * 2.0 + BOARD_SIZE;
pub const CELL_SIZE: f32 = BOARD_SIZE / GRID_SIZE;
pub const BALL_RADIUS: f32 = CELL_SIZE as f32 / 2.0;

// Colors
pub const BACKGROUND: Color = Color::new(0.192, 0.211, 0.247, 1.0);
pub const DAY: Color = Color::new(0.462, 0.670, 0.682, 1.0);
pub const NIGHT: Color = Color::new(0.933, 0.933, 0.933, 1.0);

// Physic / Rates
pub const REFRESH_RATE: f32 = 1.0 / 120.0;
