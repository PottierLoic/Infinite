use crate::constants::{BORDER_SIZE, CELL_SIZE, SPEED};

pub struct Ball {
  pub x: f32,
  pub y: f32,
  pub dx: f32,
  pub dy: f32,
  pub radius: f32,
  pub color: u32,
  pub opponent: u32,
}

impl Ball {
  pub fn new(x: f32, y: f32, dx: f32, dy: f32, color: u32, opponent: u32) -> Ball {
    Ball {
      x, y,
      dx, dy,
      radius: CELL_SIZE as f32 / 2.0,
      color,
      opponent,
    }
  }
}