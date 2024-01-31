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
    self.x += self.direction.cos();
    self.y += self.direction.sin();
    // TODO: Use constants
    if self.x < 0.0 {
      self.x = 0.0;
      self.direction += 2.0 * PI / 4.0;
    } else if self.x > 15.0 {
      self.x = 15.0;
      self.direction += 2.0 * PI / 4.0;
    }

    if self.y < 0.0 {
      self.y = 0.0;
      self.direction += 2.0 * PI / 4.0;
    } else if self.y > 15.0 {
      self.y = 15.0;
      self.direction += 2.0 * PI / 4.0;
    }
  }
}