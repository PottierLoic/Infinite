use crate::*;

pub struct Board {
  pub tiles: [Color; GRID_SIZE as usize * GRID_SIZE as usize],
  pub balls: Vec<Ball>,
}

impl Board {
  pub fn new() -> Board {
    let mut tiles: [Color; (GRID_SIZE * GRID_SIZE) as usize] = [Color::default(); (GRID_SIZE * GRID_SIZE) as usize];
    tiles.iter_mut().enumerate().for_each(|(index, tile)| {
      *tile = if index as f32 % GRID_SIZE < GRID_SIZE / 2.0 { NIGHT } else { DAY };
    });

    Board {
      tiles,
      balls: vec![
        Ball::new(GRID_SIZE * CELL_SIZE / 4.0, GRID_SIZE * CELL_SIZE / 2.0, 8.0, -8.0, DAY, NIGHT),
        Ball::new((GRID_SIZE * CELL_SIZE / 4.0) * 3.0, GRID_SIZE * CELL_SIZE / 2.0, -8.0, 8.0, NIGHT, DAY)
      ],
    }
  }

  pub fn get_cell(&self, x: u32, y: u32) -> Color {
    self.tiles[(x + y * GRID_SIZE as u32) as usize]
  }

  pub fn get_scores(&self) -> (usize, usize) {
    let score_day = self.tiles.iter().filter(|&&tile| tile == DAY).count();
    let score_night = self.tiles.iter().filter(|&&tile| tile == NIGHT).count();
    (score_day, score_night)
  }

  pub fn update(&mut self) {
    for ball in &mut self.balls {
      let steps = 8;
      for step in 0..steps {
        let angle = step as f32 * (PI / 4.0);
        let x = ball.x + angle.cos() * BALL_RADIUS;
        let y = ball.y + angle.sin() * BALL_RADIUS;

        let i = (x / CELL_SIZE).floor() as u32;
        let j = (y / CELL_SIZE).floor() as u32;

        if i < GRID_SIZE as u32 && j < GRID_SIZE as u32 {
          if self.tiles[(i + j * GRID_SIZE as u32) as usize] == ball.color {
            self.tiles[(i + j * GRID_SIZE as u32) as usize] = ball.opponent;

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
    }
  }
}