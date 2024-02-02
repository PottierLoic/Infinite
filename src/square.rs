use std::f32::consts::PI;

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
    self.x += self.direction.cos() * 2.0;
    self.y += self.direction.sin() * 2.0;

    // bound check
    if self.x as u32 >= crate::constants::GRID_SIZE * crate::constants::CELL_SIZE {
      self.x = (crate::constants::GRID_SIZE * crate::constants::CELL_SIZE) as f32;
      self.direction -= PI / 4.0;
    } else if self.x <= 0.0 {
      self.x = 0.0;
      self.direction -= PI / 4.0;
    }

    if self.y as u32 >= crate::constants::GRID_SIZE * crate::constants::CELL_SIZE {
      self.y = (crate::constants::GRID_SIZE * crate::constants::CELL_SIZE) as f32;
      self.direction -= PI / 4.0;
    } else if self.y <= 0.0 {
      self.y = 0.0;
      self.direction -= PI / 4.0;
    }
  }
}