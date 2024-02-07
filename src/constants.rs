pub const CELL_SIZE: u32 = 25;
pub const GRID_SIZE: u32 = 16;

pub const SCREEN_SIZE: u32 = 800;

pub const GRID_OFFSET: u32 = (SCREEN_SIZE - GRID_SIZE as u32 * CELL_SIZE as u32) / 2;

// Colors
pub const BACKGROUND: [u8; 4] = [25, 9, 51, 0xFF];
pub const DAY: [u8; 4] = [172, 252, 217, 0xFF];
pub const NIGHT: [u8; 4] = [102, 86, 135, 0xFF];
