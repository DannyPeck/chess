use crate::board::{position::Position, Board};
use crate::piece::Side;

pub struct Game {
    pub board: Board,
    pub current_turn: Side,
    pub white_short_castle_rights: bool,
    pub white_long_castle_rights: bool,
    pub black_short_castle_rights: bool,
    pub black_long_castle_rights: bool,
    pub en_passant_target: Option<Position>,
}

impl Game {
    pub fn new(board: Board) -> Game {
        Game {
            board,
            current_turn: Side::White,
            white_short_castle_rights: true,
            white_long_castle_rights: true,
            black_short_castle_rights: true,
            black_long_castle_rights: true,
            en_passant_target: None,
        }
    }

    pub fn attempt_move(&mut self, start_position: &Position, end_position: &Position) -> bool {
        let valid = match self.board.get_piece(start_position) {
            Some(piece) if piece.side == self.current_turn => {
                self.board.move_piece(start_position, end_position)
            }
            _ => false,
        };

        if valid {
            self.current_turn = match self.current_turn {
                Side::White => Side::Black,
                Side::Black => Side::White,
            };
        }

        valid
    }
}
