// Sizes
pub const BORDER_SIZE: u32 = 50;
pub const BOARD_SIZE: u32 = 400;
pub const GRID_SIZE: u32 = 20;
pub const SCREEN_SIZE: u32 = BORDER_SIZE * 2 + BOARD_SIZE;
pub const CELL_SIZE: u32 = BOARD_SIZE / GRID_SIZE;

// Colors
pub const BACKGROUND: [u8; 4] = [25, 9, 51, 0xFF];
pub const DAY: [u8; 4] = [172, 252, 217, 0xFF];
pub const NIGHT: [u8; 4] = [102, 86, 135, 0xFF];

// Physic / Rates
pub const FRAME_RATE: f32 = 1.0 / 60.0;
pub const SPEED: f32 = 6.0;
