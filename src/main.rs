use std::collections::HashSet;

use chess::board::position::Position;
use chess::piece::{Piece, PieceType};
use chess::Game;
use chess::Side;

fn moves_to_string(moves: &HashSet<Position>) -> String {
    let mut output = String::new();

    let mut counter = 0;
    for current_move in moves {
        if counter > 0 {
            output += ", ";
        }
        output += format!("{current_move}").as_str();

        counter = counter + 1;
    }

    output
}

fn main() {
    let mut game = Game::new();

    println!("{}", game.board);
  
    game.board.add_piece(&Piece::new(PieceType::Rook, Side::White), &Position::e4());

    println!("{}", game.board);
}
