pub mod board;
pub mod fen;
pub mod game;
pub mod piece;

use board::{position::Position, MoveRequest};
use game::Game;
use piece::PromotionType;

pub fn run() {
    let board =
        fen::parse_fen("r2qkbnr/p1Pppppp/b1n5/1p6/8/8/P1PPPPPP/RNBQKBNR w KQkq - 1 5").unwrap();
    let mut game = Game::new(board);

    let moves = vec![MoveRequest::promotion(
        Position::c7(),
        Position::c8(),
        PromotionType::Queen,
    )];

    perform_moves(&mut game, moves);
}

pub fn perform_moves(game: &mut Game, move_requests: Vec<MoveRequest>) {
    println!("{}\n", fen::generate_fen(&game.board));
    println!("{}\n", game.board);

    for request in move_requests {
        match game.attempt_move(request) {
            Ok(_) => {
                println!("{}\n", fen::generate_fen(&game.board));
                println!("{}\n", game.board);
            }
            Err(error) => {
                println!("{error:?}");
                break;
            }
        }
    }
}
