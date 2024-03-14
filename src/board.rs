use crate::constants::*;
use crate::ball::*;

pub struct Board {
  pub tiles: [u8; GRID_SIZE as usize * GRID_SIZE as usize],
  pub balls: Vec<Ball>,
}

impl Board {
  pub fn new() -> Board {
    let mut tiles = [0; (GRID_SIZE * GRID_SIZE) as usize];
    for i in 0..GRID_SIZE {
      for j in 0..GRID_SIZE {
        tiles[(i * GRID_SIZE + j) as usize] = if j < GRID_SIZE / 2 { 0 } else { 1 };
      }
    }
    Board {
      tiles,
      balls: vec![Ball::new(100.0, 200.0, 0.0, 0.0, 0, 1), Ball::new(300.0, 200.0, 0.0, 0.0, 1, 0)],
    }
  }

  // TODO should return result in case of oob
  pub fn get_cell(&self, x: u32, y: u32) -> u8 {
    self.tiles[(x + y * GRID_SIZE) as usize]
  }

  pub fn set_cell(&mut self, x: u32, y: u32, value: u8) {
    self.tiles[(x + y * GRID_SIZE) as usize] = value;
  }
}