use crate::constants::*;
use crate::Color;

#[derive(Clone)]
pub struct Ball {
  pub x: f32,
  pub y: f32,
  pub dx: f32,
  pub dy: f32,
  pub radius: f32,
  pub color: Color,
  pub opponent: Color,
}

impl Ball {
  pub fn new(x: f32, y: f32, dx: f32, dy: f32, color: Color, opponent: Color) -> Ball {
    Ball {
      x,
      y,
      dx,
      dy,
      radius: CELL_SIZE / 2.0,
      color,
      opponent,
    }
  }

  pub fn update(&self) -> Ball {
    let new_dx = if self.x + self.dx > (GRID_SIZE * CELL_SIZE) - BALL_RADIUS || self.x + self.dx < BALL_RADIUS {
      -self.dx
    } else {
      self.dx
    };

    let new_dy = if self.y + self.dy > (GRID_SIZE * CELL_SIZE) - BALL_RADIUS || self.y + self.dy < BALL_RADIUS {
      -self.dy
    } else {
      self.dy
    };

    Ball {
      x: self.x + new_dx,
      y: self.y + new_dy,
      dx: new_dx,
      dy: new_dy,
      radius: self.radius,
      color: self.color,
      opponent: self.opponent,
    }
  }
}