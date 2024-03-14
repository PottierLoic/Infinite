use rand::{Rng};
use crate::constants::*;
use crate::Color;


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
      x, y,
      dx, dy,
      radius: CELL_SIZE / 2.0,
      color,
      opponent,
    }
  }

  pub fn add_random(&mut self) {
    let mut rng = rand::thread_rng();
    self.dx += rng.gen_range(-0.005..0.005);
    self.dy += rng.gen_range(-0.005..0.005);
    self.dx = self.dx.clamp(-MAX_SPEED, MAX_SPEED);
    self.dy = self.dy.clamp(-MAX_SPEED, MAX_SPEED);
    if self.dx.abs() < MIN_SPEED {
    self.dx = MIN_SPEED * self.dx.signum();
    }
    if self.dy.abs() < MIN_SPEED {
      self.dy = MIN_SPEED * self.dy.signum();
    }
  }

  pub fn check_boundary_coll(&mut self) {
    if self.x + self.dx > (GRID_SIZE * CELL_SIZE) - BALL_RADIUS || self.x + self.dx < BALL_RADIUS {
      self.dx = -self.dx;
    }

    if self.y + self.dy > (GRID_SIZE * CELL_SIZE) - BALL_RADIUS || self.y + self.dy < BALL_RADIUS {
      self.dy = -self.dy;
    }
  }
}