use fermium::{
  prelude::{
      SDL_CreateRenderer, SDL_Delay, SDL_RenderClear, SDL_RenderPresent,
      SDL_SetRenderDrawColor
  }, renderer::SDL_Renderer, video::*, *
};

mod constants;

mod board;
use board::Board;

mod square;
use square::Square;

fn draw_game(renderer: &SDL_Renderer, board: &Board) {

}

fn main() {

  let mut board = Board::new();
  board.square_1.x = 150.0;

  unsafe {
    assert_eq!(SDL_Init(SDL_INIT_EVERYTHING), 0);
 
    // Create window
    let window = SDL_CreateWindow(
      b"Infinite battle\0".as_ptr().cast(),
      SDL_WINDOWPOS_CENTERED,
      SDL_WINDOWPOS_CENTERED,
      800,
      600,
      (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI).0,
    );

    // Panic if window is not null
    assert!(!window.is_null());
 
    let renderer = SDL_CreateRenderer(window, -1, 1);
    
    // Panic if renderer is not null
    assert!(!renderer.is_null());

    SDL_SetRenderDrawColor(renderer, constants::BACKGROUND[0], constants::BACKGROUND[1], constants::BACKGROUND[2], constants::BACKGROUND[3]);

    loop {
      SDL_RenderClear(renderer);

      draw_game(renderer, &board);

      SDL_RenderPresent(renderer);
      SDL_Delay(10);
    }

    SDL_DestroyWindow(window);
    SDL_Quit();
  }
}
