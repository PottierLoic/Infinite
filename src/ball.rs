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
}

