use crate::*;

#[derive(Clone)]
pub struct Board {
    pub tiles: Vec<Color>,
    pub balls: Vec<Ball>,
}

impl Board {
    pub fn new() -> Board {
        let tiles = (0..(GRID_SIZE * GRID_SIZE) as u32)
            .map(|index| {
                if index as f32 % GRID_SIZE < GRID_SIZE / 2.0 {
                    NIGHT
                } else {
                    DAY
                }
            })
            .collect::<Vec<Color>>();

        Board {
            tiles,
            balls: vec![
                Ball::new(
                    GRID_SIZE * CELL_SIZE / 4.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    4.0,
                    -4.0,
                    DAY,
                    NIGHT,
                ),
                Ball::new(
                    (GRID_SIZE * CELL_SIZE / 4.0) * 3.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    -4.0,
                    4.0,
                    NIGHT,
                    DAY,
                ),
                Ball::new(
                    GRID_SIZE * CELL_SIZE / 4.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    4.0,
                    -4.0,
                    DAY,
                    NIGHT,
                ),
                Ball::new(
                    (GRID_SIZE * CELL_SIZE / 4.0) * 3.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    -4.0,
                    4.0,
                    NIGHT,
                    DAY,
                ),
                Ball::new(
                    GRID_SIZE * CELL_SIZE / 4.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    4.0,
                    -4.0,
                    DAY,
                    NIGHT,
                ),
                Ball::new(
                    (GRID_SIZE * CELL_SIZE / 4.0) * 3.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    -4.0,
                    4.0,
                    NIGHT,
                    DAY,
                ),
                Ball::new(
                    GRID_SIZE * CELL_SIZE / 4.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    4.0,
                    -4.0,
                    DAY,
                    NIGHT,
                ),
                Ball::new(
                    (GRID_SIZE * CELL_SIZE / 4.0) * 3.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    -4.0,
                    4.0,
                    NIGHT,
                    DAY,
                ),
                Ball::new(
                    GRID_SIZE * CELL_SIZE / 4.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    4.0,
                    -4.0,
                    DAY,
                    NIGHT,
                ),
                Ball::new(
                    (GRID_SIZE * CELL_SIZE / 4.0) * 3.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    -4.0,
                    4.0,
                    NIGHT,
                    DAY,
                ),
                Ball::new(
                    GRID_SIZE * CELL_SIZE / 4.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    4.0,
                    -4.0,
                    DAY,
                    NIGHT,
                ),
                Ball::new(
                    (GRID_SIZE * CELL_SIZE / 4.0) * 3.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    -4.0,
                    4.0,
                    NIGHT,
                    DAY,
                ),
                Ball::new(
                    GRID_SIZE * CELL_SIZE / 4.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    4.0,
                    -4.0,
                    DAY,
                    NIGHT,
                ),
                Ball::new(
                    (GRID_SIZE * CELL_SIZE / 4.0) * 3.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    -4.0,
                    4.0,
                    NIGHT,
                    DAY,
                ),
                Ball::new(
                    GRID_SIZE * CELL_SIZE / 4.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    4.0,
                    -4.0,
                    DAY,
                    NIGHT,
                ),
                Ball::new(
                    (GRID_SIZE * CELL_SIZE / 4.0) * 3.0,
                    GRID_SIZE * CELL_SIZE / 2.0,
                    -4.0,
                    4.0,
                    NIGHT,
                    DAY,
                ),
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
            let (mut new_dx, mut new_dy) = (ball.dx, ball.dy);

            for step in 0..8 {
                let angle = step as f32 * (PI / 4.0);
                let x = ball.x + angle.cos() * BALL_RADIUS;
                let y = ball.y + angle.sin() * BALL_RADIUS;

                let i = (x / CELL_SIZE).floor() as u32;
                let j = (y / CELL_SIZE).floor() as u32;

                if i < GRID_SIZE as u32 && j < GRID_SIZE as u32 {
                    let tile_index = (i + j * GRID_SIZE as u32) as usize;
                    if self.tiles[tile_index] == ball.color {
                        self.tiles[tile_index] = ball.opponent;

                        new_dx = if angle.cos().abs() > angle.sin().abs() {
                            -new_dx
                        } else {
                            new_dx
                        };
                        new_dy = if angle.cos().abs() <= angle.sin().abs() {
                            -new_dy
                        } else {
                            new_dy
                        };
                    }
                }
            }

            if ball.x - BALL_RADIUS < 0.0 {
                new_dx = -new_dx;
            } else if ball.x + BALL_RADIUS > (GRID_SIZE as f32 * CELL_SIZE) {
                new_dx = -new_dx;
            }

            if ball.y - BALL_RADIUS < 0.0 {
                new_dy = -new_dy;
            } else if ball.y + BALL_RADIUS > (GRID_SIZE as f32 * CELL_SIZE) {
                new_dy = -new_dy;
            }

            ball.x += new_dx;
            ball.y += new_dy;
            ball.dx = new_dx;
            ball.dy = new_dy;
        }
    }
}
