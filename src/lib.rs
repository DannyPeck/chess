pub mod board;
pub mod fen;
pub mod game;
pub mod piece;

use board::{position::Position, Board, MoveRequest};
use game::Game;

pub fn run() {
    let board = Board::default();
    let mut game = Game::new(board);

    let moves = vec![
        MoveRequest::new(Position::e2(), Position::e3()),
        MoveRequest::new(Position::f7(), Position::f6()),
        MoveRequest::new(Position::f2(), Position::f4()),
        MoveRequest::new(Position::g7(), Position::g5()),
        MoveRequest::new(Position::d1(), Position::h5()),
    ];

    perform_moves(&mut game, moves);
}

pub fn perform_moves(game: &mut Game, move_requests: Vec<MoveRequest>) {
    println!("{}\n", game.board);

    for request in move_requests {
        match game.attempt_move(request) {
            Ok(_) => {
                println!("{}\n", game.board);
                println!("{:?}\n", board::get_move_state(&game.board));
            }
            Err(error) => {
                println!("{error:?}");
                break;
            }
        }
    }
}
