pub mod board;
pub mod game;
pub mod piece;

use board::{position::Position, Board};
use game::Game;
use piece::{Piece, PieceType, Side};

pub fn run() {
    let board = Board::default();

    let mut game = Game::new(board);

    println!("{}", game.board);

    game.board
        .add_piece(Piece::new(PieceType::Rook, Side::White), Position::e4());

    println!("{}", game.board);
}
