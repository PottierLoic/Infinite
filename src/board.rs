use crate::*;

#[derive(Clone)]
pub struct Board {
  pub tiles: Vec<Color>,
  pub balls: Vec<Ball>,
}

impl Board {
  pub fn new() -> Board {
    let tiles = (0..(GRID_SIZE * GRID_SIZE) as u32)
      .map(|index| if index as f32 % GRID_SIZE < GRID_SIZE / 2.0 { NIGHT } else { DAY })
      .collect::<Vec<Color>>();

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

  // pub fn update(&self) -> Board {
  //   let mut new_board = Board {
  //     tiles: self.tiles.clone(),
  //     balls: self.balls.clone(),
  //   };

  //   for ball in &mut new_board.balls {
  //     let steps = 8;
  //     for step in 0..steps {
  //       let angle = step as f32 * (PI / 4.0);
  //       let x = ball.x + angle.cos() * BALL_RADIUS;
  //       let y = ball.y + angle.sin() * BALL_RADIUS;

  //       let i = (x / CELL_SIZE).floor() as u32;
  //       let j = (y / CELL_SIZE).floor() as u32;

  //       if i < GRID_SIZE as u32 && j < GRID_SIZE as u32 {
  //         if new_board.tiles[(i + j * GRID_SIZE as u32) as usize] == ball.color {
  //           new_board.tiles[(i + j * GRID_SIZE as u32) as usize] = ball.opponent;

  //           if angle.cos().abs() > angle.sin().abs() {
  //             ball.dx = -ball.dx;
  //           } else {
  //             ball.dy = -ball.dy;
  //           }
  //         }
  //       }
  //     }
  //   }

  //   new_board.balls[0] = new_board.balls[0].update();
  //   new_board.balls[1] = new_board.balls[1].update();

  //   return new_board;
  // }

  pub fn update(&self) -> Board {
    // TODO: Remove this to have pure fucntion
    let mut updated_tiles = self.tiles.clone();
    let updated_balls = self.balls.iter().enumerate().map(|(_, ball)| {
      let (new_dx, new_dy) = (0..8).fold((ball.dx, ball.dy), |(dx, dy), step| {
        let angle = step as f32 * (PI / 4.0);
        let x = ball.x + angle.cos() * BALL_RADIUS;
        let y = ball.y + angle.sin() * BALL_RADIUS;

        let i = (x / CELL_SIZE).floor() as u32;
        let j = (y / CELL_SIZE).floor() as u32;

        if i < GRID_SIZE as u32 && j < GRID_SIZE as u32 {
          let tile_index = (i + j * GRID_SIZE as u32) as usize;
          if updated_tiles[tile_index] == ball.color {
            updated_tiles[tile_index] = ball.opponent;
            let new_dx = if angle.cos().abs() > angle.sin().abs() { -dx } else { dx };
            let new_dy = if angle.cos().abs() <= angle.sin().abs() { -dy } else { dy };
            (new_dx, new_dy)
          } else {
            (dx, dy)
          }
        } else {
          (dx, dy)
        }
      });

      Ball {
        x: ball.x + new_dx,
        y: ball.y + new_dy,
        dx: new_dx,
        dy: new_dy,
        ..*ball
      }
    }).collect::<Vec<Ball>>();

    Board {
      tiles: updated_tiles,
      balls: updated_balls.iter().map(|ball| ball.update()).collect::<Vec<Ball>>(),
    }
  }
}