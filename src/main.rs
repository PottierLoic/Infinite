mod board;
mod square;

use board::Board;

fn main() {
  let test = Board::new();
  test.print();
}
