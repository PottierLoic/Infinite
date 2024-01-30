pub struct Square {
  pub x: u8,
  pub y: u8,
  pub direction: f32,
}

impl Square {
  pub fn new(x: u8, y: u8, direction: f32) -> Square {
    Square {
      x: x,
      y: y,
      direction: direction,
    }
  }
}