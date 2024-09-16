use macroquad::{miniquad::conf, prelude::*};

use std::f32::consts::PI;

mod constants;
use constants::*;
mod board;
use board::*;
mod ball;
use ball::*;

fn window_conf() -> conf::Conf {
    conf::Conf {
        window_title: "Infinite Battle".to_owned(),
        window_width: SCREEN_SIZE as i32,
        window_height: SCREEN_SIZE as i32,
        ..Default::default()
    }
}

fn draw_board(board: &Board) {
    (0..GRID_SIZE as u32 * GRID_SIZE as u32).for_each(|i| {
        let (x, y) = (i % GRID_SIZE as u32, i / GRID_SIZE as u32);
        draw_rectangle(
            x as f32 * CELL_SIZE + BORDER_SIZE,
            y as f32 * CELL_SIZE + BORDER_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            board.get_cell(x, y),
        );
    });
}

fn draw_ball(ball: &Ball) {
    draw_circle(
        BORDER_SIZE + ball.x,
        BORDER_SIZE + ball.y,
        BALL_RADIUS,
        ball.color,
    );
}

fn draw_scores(board: &Board) {
    let (score_day, score_night) = board.get_scores();
    let day_text_width = measure_text(score_day.to_string().as_str(), None, 30.0 as u16, 1.0).width;
    let night_text_width =
        measure_text(score_night.to_string().as_str(), None, 30.0 as u16, 1.0).width;
    draw_text(
        score_day.to_string().as_str(),
        (SCREEN_SIZE / 4.0) - (day_text_width / 2.0),
        SCREEN_SIZE - BORDER_SIZE / 2.0,
        30.0,
        DAY,
    );
    draw_text(
        score_night.to_string().as_str(),
        (3.0 * SCREEN_SIZE / 4.0) - (night_text_width / 2.0),
        SCREEN_SIZE - BORDER_SIZE / 2.0,
        30.0,
        NIGHT,
    );
}

fn draw_game(board: &Board) {
    draw_board(&board);
    for ball in &board.balls {
        draw_ball(ball);
    }
    draw_scores(&board);
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board = Board::new();
    let mut last_update_time = get_time();
    loop {
        let now = get_time();
        let delta_time = now - last_update_time;
        if delta_time >= REFRESH_RATE as f64 {
            board.update();
            last_update_time = now;
        }
        clear_background(BACKGROUND);
        draw_game(&board);
        next_frame().await
    }
}
