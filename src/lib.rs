pub mod board;
pub mod fen;
pub mod game;
pub mod piece;

use board::{position::Position, MoveRequest};
use game::Game;
use piece::PromotionType;

pub fn run() {
    let parsed_board =
        fen::parse_fen("r2qkbnr/p1Pppppp/b1n5/1p6/8/8/P1PPPPPP/RNBQKBNR w KQkq - 1 5").unwrap();
    let mut parsed_game = Game::new(parsed_board);

    let parsed_moves = vec![MoveRequest::promotion(
        Position::c7(),
        Position::c8(),
        PromotionType::Knight,
    )];

    perform_moves(&mut parsed_game, parsed_moves);
}

pub fn perform_moves(game: &mut Game, move_requests: Vec<MoveRequest>) {
    for request in move_requests {
        match game.attempt_move(request) {
            Ok(_) => {
                println!("{}\n", game.board);
                println!("{}\n", fen::generate_fen(&game.board));
            }
            Err(error) => {
                println!("{error:?}");
                break;
            }
        }
    }
}
