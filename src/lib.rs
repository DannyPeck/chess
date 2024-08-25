pub mod board;
pub mod fen;
pub mod game;
pub mod piece;

use board::{position::Position, Board};
use game::Game;

pub fn run() {
    let mut game = Game::new(Board::default());

    println!("{}\n", game.board);

    let moves = vec![
        ("e2", "e4"),
        ("c7", "c6"),
        ("d2", "d4"),
        ("d7", "d5"),
        ("e4", "d5"),
        ("c6", "d5"),
        ("b1", "c3"),
        ("c8", "f5"),
        ("d1", "f3"),
        ("b8", "c6"),
        ("a1", "b1"),
        ("d8", "d7"),
        ("g2", "g4"),
        ("h7", "h6"),
        ("g4", "f5"),
    ];

    perform_moves(&mut game, &moves);

    let current_fen = fen::generate_fen(&game.board);
    println!("{current_fen}");

    let parsed_board =
        fen::parse_fen("rnbqkbnr/1pp1pppp/p7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3")
            .unwrap();
    let mut parsed_game = Game::new(parsed_board);

    println!("Parsed Game:\n{}\n", parsed_game.board);

    let parsed_moves = vec![
        ("e5", "d6"),
    ];

    perform_moves(&mut parsed_game, &parsed_moves);
}

pub fn perform_moves(game: &mut Game, moves: &Vec<(&str, &str)>) {
  for (start, end) in moves {
    match (Position::from_str(start), &Position::from_str(end)) {
        (Some(start), Some(end)) => {
            let valid = game.attempt_move(&start, &end);

            if valid {
                println!("{}\n", game.board);
                println!("{}\n", fen::generate_fen(&game.board));
            } else {
                break;
            }
        }
        _ => break,
    }
}
}