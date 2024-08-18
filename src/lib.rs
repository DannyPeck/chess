pub mod board;
pub mod game;
pub mod piece;

use board::Board;
use game::Game;

pub fn run() {
    let board = Board::default();

    let game = Game::new(board);

    println!("{}", game.board);
}
