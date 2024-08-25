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

    for (start, end) in moves {
        match (Position::from_str(start), &Position::from_str(end)) {
            (Some(start), Some(end)) => {
                let valid = game.attempt_move(&start, &end);

                if valid {
                    println!("{}\n", game.board);
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    let parsed_board =
        fen::parse_fen("rnbqkbn1/1p1p1p2/6pr/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R w Kq - 0 7")
            .unwrap();
    let parsed_game = Game::new(parsed_board);

    println!("{}", parsed_game.board);
}
