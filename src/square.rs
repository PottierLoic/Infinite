use rand::Rng;

#[derive(Copy, Clone)]
pub struct Square {
  pub id: u32,
  pub x: f32,
  pub y: f32,
  pub direction: f32,
}

impl Square {
  pub fn new(id: u32, x: f32, y: f32, direction: f32) -> Square {
    Square {id, x, y, direction }
  }

  pub fn _print(&self) {
    println!("({}, {}) {}", self.x, self.y, self.direction);
  }

  pub fn bounce_x(&mut self) {
    self.direction = std::f32::consts::PI - self.direction;
    self.normalize_direction();
    self.add_randomness();
  }

  pub fn bounce_y(&mut self) {
    self.direction = 2.0 * std::f32::consts::PI - self.direction;
    self.normalize_direction();
    self.add_randomness();
  }

  pub fn normalize_direction(&mut self) {
    self.direction = self.direction % (2.0 * std::f32::consts::PI);
    if self.direction < 0.0 {
      self.direction += 2.0 * std::f32::consts::PI;
    }
  }

  pub fn add_randomness(&mut self) {
    let mut rng = rand::thread_rng();
    let random_direction = rng.gen_range(-0.005..0.005);
    self.direction += random_direction;
    self.normalize_direction();
  }
}