use pixels::{Error, Pixels, SurfaceTexture};

use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod constants;

mod board;
use board::Board;

mod square;
use square::Square;

fn draw_background(frame: &mut [u8]) {
  for (_, pixel) in frame.chunks_exact_mut(4).enumerate() {
    pixel.copy_from_slice(&constants::BACKGROUND);
  }
}

fn draw_square(cell: &Square, frame: &mut [u8]) {
  for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
    let x = i % constants::SCREEN_SIZE as usize;
    let y = i / constants::SCREEN_SIZE as usize;
    if x >= cell.x as usize && x < cell.x as usize + constants::CELL_SIZE && y >= cell.y as usize && y < cell.y as usize + constants::CELL_SIZE {
      pixel.copy_from_slice(&cell.color);
    }
  }
}

fn main() {
  let mut input = WinitInputHelper::new();
  let event_loop = EventLoop::new();
  let size = LogicalSize::new(constants::SCREEN_SIZE, constants::SCREEN_SIZE);
  let window = WindowBuilder::new()
    .with_title("Infinite battle")
    .with_inner_size(size)
    .with_min_inner_size(size)
    .build(&event_loop)
    .unwrap();

  let mut pixels = {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    Pixels::new(constants::SCREEN_SIZE as u32, constants::SCREEN_SIZE as u32, surface_texture).unwrap()
  };

  let mut board = Board::new();
  board.square_1.x = 150.0;

  event_loop.run(move | event, _, control_flow| {
    if let Event::RedrawRequested(_) = event {
      draw_background(pixels.frame_mut());
      draw_square(&board.square_1, pixels.frame_mut());
      draw_square(&board.square_2, pixels.frame_mut());
      if pixels.render().is_err() {
        *control_flow = ControlFlow::Exit;
        return;
      }
    }

    if input.update(&event) {
      if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
        *control_flow = ControlFlow::Exit;
        return;
      }
    }
  })

  // let mut test = Board::new();
  // test.print();

  // for _ in 0..10 {
  //   test.update();
  //   test.print();
  //   test.square_1._print();
  //   test.square_2._print();
  // }
}