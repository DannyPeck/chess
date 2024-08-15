// Ok, so to make a chess game there has to be a few things to consider
// - The board
// - The different pieces
// - Player's and their pieces
// - Identifying legal moves
// - Choosing the best move
// - Turns

// Ok, the problem I have now is that I am tracking the piece position twice, which sucks.
// For simple player vs. player rules, I really only need to check that the user is moving their own piece to another valid position.
// Enumerating the other valid positions is only important when implementing a bot that is assigning value to each of them.

use chess::board::position::Position;
use chess::piece::{Piece, PieceType};
use chess::Side;
use chess::Game;

fn moves_to_string(moves: &Vec<Position>) -> String {
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

    game.board.print_board();

    game.move_piece(&Position::b2(), &Position::b3());
    game.move_piece(&Position::e7(), &Position::e6());

    game.board.print_board();
}
