use crate::{constants::{self}, square::Square};
use std::f32::consts::PI;


pub struct Board {
  pub tiles: [u8; constants::GRID_SIZE as usize * constants::GRID_SIZE as usize],
  pub square_1: Square,
  pub square_2: Square,
}

impl Board {
  pub fn new() -> Board {
    let mut tiles = [0; (constants::GRID_SIZE * constants::GRID_SIZE) as usize];
    for i in 0..constants::GRID_SIZE {
      for j in 0..constants::GRID_SIZE {
        tiles[(i * constants::GRID_SIZE + j) as usize] = if i < constants::GRID_SIZE / 2 { 1 } else { 1 };
      }
    }
    Board {
      tiles,
      square_1: Square::new(0, 100.0, 200.0, 2.0 * PI / 3.0),
      square_2: Square::new(1, 300.0, 200.0, 5.0 *  (PI / 3.0)),
    }
  }

  // TODO should return result in case of oob
  pub fn get_cell(&self, x: u32, y: u32) -> u8 {
    self.tiles[(x * constants::GRID_SIZE + y) as usize]
  }

  pub fn update(&mut self) {
    self.square_1.x += self.square_1.direction.cos() * constants::SPEED;
    if self.square_1.x < 0.0 {
      self.square_1.x = 0.0;
      self.square_1.bounce_x();
    } else if self.square_1.x > (constants::GRID_SIZE - 1) as f32 * constants::CELL_SIZE as f32 {
      self.square_1.x = (constants::GRID_SIZE - 1) as f32 * constants::CELL_SIZE as f32;
      self.square_1.bounce_x();
    }

    self.square_1.y += self.square_1.direction.sin() * constants::SPEED;
    if self.square_1.y < 0.0 {
      self.square_1.y = 0.0;
      self.square_1.bounce_y();
    } else if self.square_1.y > (constants::GRID_SIZE - 1) as f32 * constants::CELL_SIZE as f32 {
      self.square_1.y = (constants::GRID_SIZE - 1) as f32 * constants::CELL_SIZE as f32;
      self.square_1.bounce_y();
    }
  }
}