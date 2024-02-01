use crate::{constants, square::Square};


pub struct Board {
  pub tiles: [u8; constants::GRID_SIZE * constants::GRID_SIZE],
  pub square_1: Square,
  pub square_2: Square,
}

impl Board {
  pub fn new() -> Board {
    let mut tiles = [0; constants::GRID_SIZE * constants::GRID_SIZE];
    for i in 0..constants::GRID_SIZE {
      for j in 0..constants::GRID_SIZE {
        tiles[i * constants::GRID_SIZE + j] = if j < 8 { 1 } else { 0 };
      }
    }
    Board {
      tiles: tiles,
      square_1: Square::new(0.0, 0.0, 0.0, constants::DAY),
      square_2: Square::new(0.0, 0.0, 0.0, constants::NIGHT),
    }
  }

  pub fn print(&self) {
    for i in 0..constants::GRID_SIZE {
      for j in 0..constants::GRID_SIZE {
        print!("{} ", if self.get_cell(i, j) == 0 { "o" } else { "*" });
      }
      println!();
    }
    println!("\n   Day : {}  |  Night : {}\n", self.get_amount(0), self.get_amount(1));
  }

  pub fn get_amount(&self, square: u8) -> u8 {
    let mut amount = 0;
    for i in 0..constants::GRID_SIZE {
      for j in 0..constants::GRID_SIZE {
        if self.get_cell(i, j) == square {
          amount += 1;
        }
      }
    }
    amount
  }

  pub fn get_cell(&self, x: usize, y: usize) -> u8 {
    self.tiles[x * constants::GRID_SIZE + y]
  }

  pub fn update(&mut self) {
    self.square_1.update();
    self.square_2.update();
  }
}