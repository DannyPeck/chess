pub mod board;
pub mod fen;
pub mod game;
pub mod piece;

use board::{position::Position, Board};
use game::Game;

pub fn run() {
    let parsed_board =
        fen::parse_fen("rnbqkbnr/pppp1ppp/4p3/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
    let mut parsed_game = Game::new(parsed_board);

    println!("Parsed Game:\n{}\n", parsed_game.board);
    println!("{:#?}\n", parsed_game.board.get_castle_rights());

    let parsed_moves = vec![("e1", "e2"), ("e8", "e7")];

    perform_moves(&mut parsed_game, &parsed_moves);
}

pub fn perform_moves(game: &mut Game, moves: &Vec<(&str, &str)>) {
    for (start, end) in moves {
        match (Position::from_str(start), &Position::from_str(end)) {
            (Some(start), Some(end)) => {
                let result = game.attempt_move(&start, &end);

                match result {
                  Ok(_) => {
                    println!("{}\n", game.board);
                    println!("{}\n", fen::generate_fen(&game.board));
                  },
                  Err(error) => {
                    println!("{error:?}");
                    break;
                  }
                }
            }
            _ => break,
        }
    }
}
