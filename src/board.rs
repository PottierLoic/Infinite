use crate::square::Square;

pub struct Board {
  pub tiles: [[u8; 16]; 16],
  pub square_1: Square,
  pub square_2: Square,
}

impl Board {
  pub fn new() -> Board {
    Board {
      tiles: [[0; 16]; 16],
      square_1: Square::new(0, 0, 0.0),
      square_2: Square::new(0, 0, 0.0),
    }
  }

  pub fn print(self) {
    for i in 0..16 {
      for j in 0..16 {
        print!("{}", if self.tiles[i][j] == 0 { "o" } else { "*" });
      }
      println!("");
    }
  }
}