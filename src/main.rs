use macroquad::{miniquad::conf, prelude::*};

mod constants;
use constants::*;
mod board;
use board::*;
mod ball;
use ball::*;

fn window_conf () -> conf::Conf {
  conf::Conf {
    window_title: "Infinite Battle".to_owned(),
    window_width: SCREEN_SIZE as i32,
    window_height: SCREEN_SIZE as i32,
    ..Default::default()
  }
}

fn draw_board(board: &Board) {
  for x in 0..GRID_SIZE {
    for y in 0..GRID_SIZE {
      let cell = board.get_cell(x, y);
      let color = if cell == 0 { DAY } else { NIGHT };
      draw_rectangle(
        BORDER_SIZE as f32 + x as f32 * CELL_SIZE as f32,
        BORDER_SIZE as f32 + y as f32 * CELL_SIZE as f32,
        CELL_SIZE as f32,
        CELL_SIZE as f32,
        Color::from(color),
      );
    }
  }
}

fn draw_ball(ball: &Ball) {
  draw_circle(
    BORDER_SIZE as f32 + ball.x,
    BORDER_SIZE as f32 + ball.y,
    BALL_RADIUS,
    Color::from(if ball.color == 0 { NIGHT } else { DAY }));
}

#[macroquad::main(window_conf)]
async fn main() {
  let mut board = Board::new();
  loop {
    clear_background(BACKGROUND);
    draw_board(&board);
    for ball in &board.balls {
      draw_ball(ball);
    }
    next_frame().await
  }
}