pub mod board;
pub mod fen;
pub mod game;
pub mod piece;

use board::MoveRequest;
use game::Game;

pub fn run() {
    let board =
        fen::parse_fen("rnb1kbnr/pp1ppppp/8/2p5/3P4/1P3N2/P3PPPP/q2QKBNR w Kkq - 0 6").unwrap();
    let mut game = Game::new(board);

    println!("{}\n", fen::generate_fen(&game.board));
    println!("{}\n", game.board);
    println!("{}\n", game.get_relative_score());
}

pub fn perform_moves(game: &mut Game, move_requests: Vec<MoveRequest>) {
    println!("{}\n", fen::generate_fen(&game.board));
    println!("{}\n", game.board);

    for request in move_requests {
        match game.attempt_move(request) {
            Ok(_) => {
                println!("{}\n", fen::generate_fen(&game.board));
                println!("{}\n", game.board);
                println!("{}\n", game.get_relative_score());
            }
            Err(error) => {
                println!("{error:?}");
                break;
            }
        }
    }
}
