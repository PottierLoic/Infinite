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

    let left_x = (self.square_1.x - constants::CELL_SIZE as f32 / 2.0) / constants::CELL_SIZE as f32;
    let right_x = (self.square_1.x + constants::CELL_SIZE as f32 / 2.0) / constants::CELL_SIZE as f32;
    let top_y = (self.square_1.y - constants::CELL_SIZE as f32 / 2.0) / constants::CELL_SIZE as f32;
    let bottom_y = (self.square_1.y + constants::CELL_SIZE as f32 / 2.0) / constants::CELL_SIZE as f32;

    let center_x = (self.square_1.x / constants::CELL_SIZE as f32) as u32;
    let center_y = (self.square_1.y / constants::CELL_SIZE as f32) as u32;

    let mut collided_list: Vec<(u32, u32)> = Vec::new();
    for x in left_x as u32..=right_x as u32 {
      for y in top_y as u32..=bottom_y as u32 {
        // check bounds 
        if x >= constants::GRID_SIZE || y >= constants::GRID_SIZE {
          continue;
        }
        if self.get_cell(y, x) == 0 {
          self.tiles[(y * constants::GRID_SIZE + x) as usize] = 1;
          collided_list.push((x, y));
        }
      }
    }

    // Assuming collided_list is populated as shown in your code snippet
    if !collided_list.is_empty() {
      // print list 
      println!("{:?}", collided_list);
      // Initialize counters for the directions
      let mut vertical_collisions = 0;
      let mut horizontal_collisions = 0;

      // Analyze each collision point
      for &(x, y) in &collided_list {
        // Check if the collision is more horizontal or vertical relative to the square's center
        if x == center_x {
          vertical_collisions += 1; // Collision is on top or bottom
        } else if y == center_y {
          horizontal_collisions += 1; // Collision is on left or right
        }
      }

      // Determine the primary direction of collision
      if vertical_collisions > horizontal_collisions {
        // Vertical collision, adjust direction accordingly
        // This could mean bouncing back on the vertical axis, depending on your game's physics
        self.square_1.direction = 2.0 * std::f32::consts::PI - self.square_1.direction;
      } else if horizontal_collisions > vertical_collisions {
        // Horizontal collision, adjust direction accordingly
        // This could mean bouncing back on the horizontal axis
        self.square_1.direction = std::f32::consts::PI - self.square_1.direction;
      }

      // Ensure the direction is normalized
      self.square_1.direction = self.square_1.direction % (2.0 * std::f32::consts::PI);
      if self.square_1.direction < 0.0 {
        self.square_1.direction += 2.0 * std::f32::consts::PI;
      }
    }
  }
}