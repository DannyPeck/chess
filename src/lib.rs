pub mod board;
pub mod fen;
pub mod game;
pub mod piece;

use board::{Board, MoveRequest};
use game::Game;

#[derive(Debug)]
pub struct ParseError(String);

impl ParseError {
    pub fn new(error: &str) -> ParseError {
        ParseError(String::from(error))
    }
}

pub fn run() {
    let board = Board::default();
    let mut game = Game::new(board);

    let moves = vec![
        MoveRequest::from_coordinate("e2e3").unwrap(),
        MoveRequest::from_coordinate("f7f6").unwrap(),
        MoveRequest::from_coordinate("f2f4").unwrap(),
        MoveRequest::from_coordinate("g7g5").unwrap(),
        MoveRequest::from_coordinate("d1h5").unwrap(),
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
