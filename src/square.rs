use std::f32::consts::PI;
use crate::constants;

pub struct Square {
  pub x: f32,
  pub y: f32,
  pub direction: f32,
}

impl Square {
  pub fn new(x: f32, y: f32, direction: f32) -> Square {
    Square {
      x: x,
      y: y,
      direction: direction,
    }
  }

  pub fn _print(&self) {
    println!("({}, {}) {}", self.x, self.y, self.direction);
  }

  pub fn update(&mut self) {
    self.x += self.direction.cos() * 10.0;
    self.y += self.direction.sin() * 10.0;

    // bound check
    if self.x >= (constants::GRID_SIZE * constants::CELL_SIZE) as f32 - constants::CELL_SIZE as f32 * 0.5 {
      self.x = (constants::GRID_SIZE * constants::CELL_SIZE) as f32 - constants::CELL_SIZE as f32 * 0.5 ;
      self.direction -= PI / 4.0;
    } else if self.x <= constants::CELL_SIZE as f32 * 0.5 {
      self.x = constants::CELL_SIZE as f32 * 0.5;
      self.direction -= PI / 4.0;
    }

    if self.y >= (constants::GRID_SIZE * constants::CELL_SIZE) as f32 - constants::CELL_SIZE as f32 * 0.5 {
      self.y = (constants::GRID_SIZE * constants::CELL_SIZE) as f32 - constants::CELL_SIZE as f32 * 0.5;
      self.direction -= PI / 4.0;
    } else if self.y <= constants::CELL_SIZE as f32 * 0.5 {
      self.y = constants::CELL_SIZE as f32 * 0.5;
      self.direction -= PI / 4.0;
    }

    // Normalize the direction
    self.direction = self.direction % (2.0 * PI);
    if self.direction < 0.0 {
      self.direction += 2.0 * PI;
    }
  }
}