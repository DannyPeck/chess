use chess::board::position::Position;
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
