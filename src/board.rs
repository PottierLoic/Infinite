use crate::{constants, square::Square};
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
        tiles[(i * constants::GRID_SIZE + j) as usize] = if j < 8 { 1 } else { 0 };
      }
    }
    Board {
      tiles: tiles,
      square_1: Square::new(100.0, 200.0, 2.0 * PI / 3.0),
      square_2: Square::new(300.0, 200.0, 5.0 *  (PI / 3.0)),
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

  pub fn get_cell(&self, x: u32, y: u32) -> u8 {
    self.tiles[(x * constants::GRID_SIZE + y) as usize]
  }

  pub fn update(&mut self) {
    self.square_1.update();
    let x = (self.square_1.x / constants::CELL_SIZE as f32) as u32;
    let y = (self.square_1.y / constants::CELL_SIZE as f32) as u32;
    if self.get_cell(y, x) == 0 {
      self.tiles[(y * constants::GRID_SIZE + x) as usize] = 1;
      self.square_1.direction += PI / 4.0;
    }

    self.square_2.update();
    let x = (self.square_2.x / constants::CELL_SIZE as f32) as u32;
    let y = (self.square_2.y / constants::CELL_SIZE as f32) as u32;
    if self.get_cell(y, x) == 1 {
      self.tiles[(y * constants::GRID_SIZE + x) as usize] = 0;
      self.square_2.direction += PI / 4.0;
    }
  }
}