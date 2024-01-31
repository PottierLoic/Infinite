mod board;
mod square;
mod constants;

use board::Board;

use std::process::Command;


fn main() {
  let mut test = Board::new();
  test.print();

  for _ in 0..10 {
    test.update();
    Command::new("cmd").arg("/c").arg("cls").status();
    test.print();
    test.square_1._print();
    test.square_2._print();
  }
}
