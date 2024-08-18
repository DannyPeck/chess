use chess::board::position::Position;
use chess::piece::{Piece, PieceType};
use chess::Game;
use chess::Side;

fn main() {
  let mut game = Game::new();

  println!("{}", game.board);

  game.board.add_piece(Piece::new(PieceType::Rook, Side::White), Position::e4());

  println!("{}", game.board);
}
