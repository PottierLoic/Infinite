use crate::*;

pub struct Board {
  pub tiles: [Color; GRID_SIZE as usize * GRID_SIZE as usize],
  pub balls: Vec<Ball>,
}

impl Board {
  pub fn new() -> Board {
    let mut tiles = [Color{r: 0.0, g: 0.0, b: 0.0, a: 0.0}; (GRID_SIZE * GRID_SIZE) as usize];
    for i in 0..GRID_SIZE {
      for j in 0..GRID_SIZE {
        tiles[(i * GRID_SIZE + j) as usize] = if j < GRID_SIZE / 2 { NIGHT } else { DAY };
      }
    }
    Board {
      tiles,
      balls: vec![
        Ball::new(SCREEN_SIZE as f32 / 4.0, SCREEN_SIZE as f32 / 2.0, 8.0, -8.0, DAY, NIGHT),
        Ball::new((SCREEN_SIZE as f32 / 4.0) * 3.0, SCREEN_SIZE as f32 / 2.0, -8.0, 8.0, NIGHT, DAY)
      ],
    }
  }

  pub fn get_cell(&self, x: u32, y: u32) -> Color {
    self.tiles[(x + y * GRID_SIZE) as usize]
  }

  pub fn set_cell(&mut self, x: u32, y: u32, value: Color) {
    self.tiles[(x + y * GRID_SIZE) as usize] = value;
  }

  pub fn update(&mut self) {
    for ball in &mut self.balls {
      let steps = 8;
      for step in 0..steps {
        let angle = step as f64 * (PI / 4.0);
        let x = ball.x as f64 + angle.cos() * BALL_RADIUS as f64;
        let y = ball.y as f64 + angle.sin() * BALL_RADIUS as f64;

        let i = (x as f32 / CELL_SIZE as f32).floor() as u32;
        let j = (y as f32 / CELL_SIZE as f32).floor() as u32;

        if i >= 0 && i < GRID_SIZE && j >= 0 && j < GRID_SIZE {
          if self.tiles[(i + j * GRID_SIZE) as usize] == ball.color {
            self.tiles[(i + j * GRID_SIZE) as usize] = ball.opponent;

            if angle.cos().abs() > angle.sin().abs() {
              ball.dx = -ball.dx;
            } else {
              ball.dy = -ball.dy;
            }
          }
        }
      }
    }

    for ball in &mut self.balls {
      ball.check_boundary_coll();
      ball.x+=ball.dx;
      ball.y+=ball.dy;
      ball.add_random();
    }
  }
}