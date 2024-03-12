extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::time::{Duration, Instant};

mod constants;
mod square;
mod board;
use board::Board;

fn draw_board(board: &Board, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
  for idx in 0..board.tiles.len() {
    let x = idx as u32 % constants::GRID_SIZE;
    let y = idx as u32 / constants::GRID_SIZE;
    let color = match board.get_cell(y, x) {
      0 => Color::RGB(constants::DAY[0], constants::DAY[1], constants::DAY[2]),
      1 => Color::RGB(constants::NIGHT[0], constants::NIGHT[1], constants::NIGHT[2]),
      _ => Color::RGB(0, 0, 0),
    };
    let rect = Rect::new(
      (constants::BORDER_SIZE / 2 + x * constants::CELL_SIZE) as i32,
      (constants::BORDER_SIZE / 2 + y * constants::CELL_SIZE) as i32,
      constants::CELL_SIZE,
      constants::CELL_SIZE,
    );
    canvas.set_draw_color(color);
    canvas.fill_rect(rect).unwrap();
  }

  // Squares
  let square_1 = &board.square_1;
  let square_2 = &board.square_2;
  canvas.set_draw_color(Color::RGB(constants::DAY[0], constants::DAY[1], constants::DAY[2]));
  let rect1 = Rect::new(
    (constants::BORDER_SIZE / 2 + square_1.x as u32 - constants::CELL_SIZE / 2) as i32,
    (constants::BORDER_SIZE / 2 + square_1.y as u32 - constants::CELL_SIZE / 2) as i32,
    constants::CELL_SIZE,
    constants::CELL_SIZE,
  );
  canvas.fill_rect(rect1).unwrap();
  canvas.set_draw_color(Color::RGB(constants::NIGHT[0], constants::NIGHT[1], constants::NIGHT[2]));
  let rect2 = Rect::new(
    (constants::BORDER_SIZE / 2 + square_2.x as u32 - constants::CELL_SIZE / 2) as i32,
    (constants::BORDER_SIZE / 2 + square_2.y as u32 - constants::CELL_SIZE / 2) as i32,
    constants::CELL_SIZE,
    constants::CELL_SIZE,
  );
  canvas.fill_rect(rect2).unwrap();
}

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let window = video_subsystem
    .window("Infinite battle", constants::SCREEN_SIZE, constants::SCREEN_SIZE)
    .position_centered()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;

  let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
  let mut event_pump = sdl_context.event_pump()?;

  let mut board = Board::new();

  let update_rate = Duration::from_secs_f32(constants::FRAME_RATE);
  let mut accumulator = Duration::new(0, 0);
  let mut current_time = Instant::now();

  'running: loop {
    let new_time = Instant::now();
    let delta_time = new_time.duration_since(current_time);
    current_time = new_time;
    accumulator += delta_time;

    while accumulator >= update_rate {
      board.update();
      accumulator -= update_rate;
    }

    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        _ => {}
      }
    }

    canvas.set_draw_color(Color::RGB(constants::BACKGROUND[0], constants::BACKGROUND[1], constants::BACKGROUND[2]));
    canvas.clear();

    draw_board(&board, &mut canvas);

    canvas.present();
  }

  Ok(())
}
