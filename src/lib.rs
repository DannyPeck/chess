pub mod board;
pub mod piece;
pub mod game;

use board::{Board, position::Position};
use game::Game;
use piece::{Piece, PieceType, Side};

pub fn run() {
  let board = Board::default();
  
  let mut game = Game::new(board);

  println!("{}", game.board);

  game.board.add_piece(Piece::new(PieceType::Rook, Side::White), Position::e4());

  println!("{}", game.board);
}