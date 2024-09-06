pub mod file;
pub mod position;
pub mod rank;
mod utils;

pub use utils::{
    get_all_legal_moves, get_move_state, is_in_check, move_piece, MoveError, MoveKind, MoveRequest,
    MoveState,
};

use std::collections::HashSet;

use crate::{
    piece::{Piece, PieceType, Side},
    piece_position,
};
use position::Position;

const BOARD_SIZE: usize = 64;
const EMPTY: Option<Piece> = None;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct CastleRights {
    pub white_short_castle_rights: bool,
    pub white_long_castle_rights: bool,
    pub black_short_castle_rights: bool,
    pub black_long_castle_rights: bool,
}

impl CastleRights {
    pub fn new(
        white_short_castle_rights: bool,
        white_long_castle_rights: bool,
        black_short_castle_rights: bool,
        black_long_castle_rights: bool,
    ) -> CastleRights {
        CastleRights {
            white_short_castle_rights,
            white_long_castle_rights,
            black_short_castle_rights,
            black_long_castle_rights,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    positions: [Option<Piece>; BOARD_SIZE],
    white_positions: HashSet<Position>,
    black_positions: HashSet<Position>,
    current_turn: Side,
    castle_rights: CastleRights,
    en_passant_target: Option<Position>,
    half_moves: u32,
    full_moves: u32,
}

impl Board {
    pub fn empty() -> Board {
        let positions: [Option<Piece>; BOARD_SIZE] = [EMPTY; BOARD_SIZE];
        Board {
            positions,
            white_positions: HashSet::new(),
            black_positions: HashSet::new(),
            current_turn: Side::White,
            castle_rights: CastleRights {
                white_short_castle_rights: true,
                white_long_castle_rights: true,
                black_short_castle_rights: true,
                black_long_castle_rights: true,
            },
            en_passant_target: None,
            half_moves: 0,
            full_moves: 1,
        }
    }

    pub fn new(
        pieces: Vec<(Position, Piece)>,
        current_turn: Side,
        castle_rights: CastleRights,
        en_passant_target: Option<Position>,
        half_moves: u32,
        full_moves: u32,
    ) -> Board {
        let positions: [Option<Piece>; BOARD_SIZE] = [EMPTY; BOARD_SIZE];

        let mut board = Board {
            positions,
            white_positions: HashSet::new(),
            black_positions: HashSet::new(),
            current_turn,
            castle_rights,
            en_passant_target,
            half_moves,
            full_moves,
        };

        board.add_pieces(pieces);

        board
    }

    pub fn get_current_turn(&self) -> &Side {
        &self.current_turn
    }

    pub fn change_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Side::White => Side::Black,
            Side::Black => {
                self.full_moves += 1;
                Side::White
            }
        };
    }

    pub fn get_castle_rights(&self) -> &CastleRights {
        &self.castle_rights
    }

    pub fn get_en_passant_target(&self) -> &Option<Position> {
        &self.en_passant_target
    }

    pub fn get_half_moves(&self) -> u32 {
        self.half_moves
    }

    pub fn get_full_moves(&self) -> u32 {
        self.full_moves
    }

    pub fn get_white_positions(&self) -> &HashSet<Position> {
        &self.white_positions
    }

    pub fn get_black_positions(&self) -> &HashSet<Position> {
        &self.black_positions
    }

    pub fn get_piece(&self, position: &Position) -> Option<&Piece> {
        self.positions[position.value()].as_ref()
    }

    pub fn take_piece(&mut self, position: &Position) -> Option<Piece> {
        let opt_piece = self.positions[position.value()].take();

        if let Some(piece) = &opt_piece {
            match piece.side {
                Side::White => {
                    self.white_positions.remove(position);
                }
                Side::Black => {
                    self.black_positions.remove(position);
                }
            }
        }

        opt_piece
    }

    pub fn set_position(&mut self, position: &Position, opt_piece: Option<Piece>) {
        // Remove any existing piece first.
        let _ = self.take_piece(position);

        if let Some(piece) = &opt_piece {
            match piece.side {
                Side::White => {
                    self.white_positions.insert(position.clone());
                }
                Side::Black => {
                    self.black_positions.insert(position.clone());
                }
            }
        }

        self.positions[position.value()] = opt_piece;
    }

    pub fn add_piece(&mut self, position: &Position, piece: Piece) {
        self.set_position(&position, Some(piece));
    }

    pub fn add_pieces(&mut self, pieces: Vec<(Position, Piece)>) {
        for (position, piece) in pieces {
            self.add_piece(&position, piece);
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        let pieces = vec![
            piece_position!(a2, Pawn, White),
            piece_position!(b2, Pawn, White),
            piece_position!(c2, Pawn, White),
            piece_position!(d2, Pawn, White),
            piece_position!(e2, Pawn, White),
            piece_position!(f2, Pawn, White),
            piece_position!(g2, Pawn, White),
            piece_position!(h2, Pawn, White),
            piece_position!(a1, Rook, White),
            piece_position!(b1, Knight, White),
            piece_position!(c1, Bishop, White),
            piece_position!(d1, Queen, White),
            piece_position!(e1, King, White),
            piece_position!(f1, Bishop, White),
            piece_position!(g1, Knight, White),
            piece_position!(h1, Rook, White),
            piece_position!(a7, Pawn, Black),
            piece_position!(b7, Pawn, Black),
            piece_position!(c7, Pawn, Black),
            piece_position!(d7, Pawn, Black),
            piece_position!(e7, Pawn, Black),
            piece_position!(f7, Pawn, Black),
            piece_position!(g7, Pawn, Black),
            piece_position!(h7, Pawn, Black),
            piece_position!(a8, Rook, Black),
            piece_position!(b8, Knight, Black),
            piece_position!(c8, Bishop, Black),
            piece_position!(d8, Queen, Black),
            piece_position!(e8, King, Black),
            piece_position!(f8, Bishop, Black),
            piece_position!(g8, Knight, Black),
            piece_position!(h8, Rook, Black),
        ];

        let mut board = Board::empty();

        board.add_pieces(pieces);

        board
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_string = String::new();
        for rank in (rank::ONE..=rank::EIGHT).rev() {
            let mut rank_string = String::new();
            for file in file::A..=file::H {
                let position = Position::from_file_and_rank(file, rank);
                let piece_notation = match self.get_piece(&position) {
                    Some(piece) => piece.to_string(),
                    None => String::from(" "),
                };

                let position_string = format!("[{piece_notation}]");
                rank_string.push_str(&position_string);
            }

            board_string.push_str(&rank_string);

            if rank != rank::ONE {
                board_string.push_str("\n");
            }
        }

        write!(f, "{board_string}")
    }
}

#[cfg(test)]
mod tests {
    use crate::board_position;

    use super::*;

    #[test]
    fn default_test() {
        let board = Board::default();

        let position_tests: Vec<(Position, Option<Piece>)> = vec![
            board_position!(a1, Rook, White),
            board_position!(b1, Knight, White),
            board_position!(c1, Bishop, White),
            board_position!(d1, Queen, White),
            board_position!(e1, King, White),
            board_position!(f1, Bishop, White),
            board_position!(g1, Knight, White),
            board_position!(h1, Rook, White),
            board_position!(a2, Pawn, White),
            board_position!(b2, Pawn, White),
            board_position!(c2, Pawn, White),
            board_position!(d2, Pawn, White),
            board_position!(e2, Pawn, White),
            board_position!(f2, Pawn, White),
            board_position!(g2, Pawn, White),
            board_position!(h2, Pawn, White),
            board_position!(a3, None),
            board_position!(b3, None),
            board_position!(c3, None),
            board_position!(d3, None),
            board_position!(e3, None),
            board_position!(f3, None),
            board_position!(g3, None),
            board_position!(h3, None),
            board_position!(a4, None),
            board_position!(b4, None),
            board_position!(c4, None),
            board_position!(d4, None),
            board_position!(e4, None),
            board_position!(f4, None),
            board_position!(g4, None),
            board_position!(h4, None),
            board_position!(a5, None),
            board_position!(b5, None),
            board_position!(c5, None),
            board_position!(d5, None),
            board_position!(e5, None),
            board_position!(f5, None),
            board_position!(g5, None),
            board_position!(h5, None),
            board_position!(a6, None),
            board_position!(b6, None),
            board_position!(c6, None),
            board_position!(d6, None),
            board_position!(e6, None),
            board_position!(f6, None),
            board_position!(g6, None),
            board_position!(h6, None),
            board_position!(a7, Pawn, Black),
            board_position!(b7, Pawn, Black),
            board_position!(c7, Pawn, Black),
            board_position!(d7, Pawn, Black),
            board_position!(e7, Pawn, Black),
            board_position!(f7, Pawn, Black),
            board_position!(g7, Pawn, Black),
            board_position!(h7, Pawn, Black),
            board_position!(a8, Rook, Black),
            board_position!(b8, Knight, Black),
            board_position!(c8, Bishop, Black),
            board_position!(d8, Queen, Black),
            board_position!(e8, King, Black),
            board_position!(f8, Bishop, Black),
            board_position!(g8, Knight, Black),
            board_position!(h8, Rook, Black),
        ];

        for (position, piece) in position_tests {
            assert_eq!(board.get_piece(&position), piece.as_ref());
        }

        assert_eq!(*board.get_current_turn(), Side::White);

        assert_eq!(
            *board.get_castle_rights(),
            CastleRights::new(true, true, true, true)
        );

        assert_eq!(*board.get_en_passant_target(), None);

        assert_eq!(board.get_half_moves(), 0);

        assert_eq!(board.get_full_moves(), 1);
    }

    #[test]
    fn empty_test() {
        let board = Board::empty();

        let position_tests: Vec<(Position, Option<Piece>)> = vec![
            board_position!(a1, None),
            board_position!(b1, None),
            board_position!(c1, None),
            board_position!(d1, None),
            board_position!(e1, None),
            board_position!(f1, None),
            board_position!(g1, None),
            board_position!(h1, None),
            board_position!(a2, None),
            board_position!(b2, None),
            board_position!(c2, None),
            board_position!(d2, None),
            board_position!(e2, None),
            board_position!(f2, None),
            board_position!(g2, None),
            board_position!(h2, None),
            board_position!(a3, None),
            board_position!(b3, None),
            board_position!(c3, None),
            board_position!(d3, None),
            board_position!(e3, None),
            board_position!(f3, None),
            board_position!(g3, None),
            board_position!(h3, None),
            board_position!(a4, None),
            board_position!(b4, None),
            board_position!(c4, None),
            board_position!(d4, None),
            board_position!(e4, None),
            board_position!(f4, None),
            board_position!(g4, None),
            board_position!(h4, None),
            board_position!(a5, None),
            board_position!(b5, None),
            board_position!(c5, None),
            board_position!(d5, None),
            board_position!(e5, None),
            board_position!(f5, None),
            board_position!(g5, None),
            board_position!(h5, None),
            board_position!(a6, None),
            board_position!(b6, None),
            board_position!(c6, None),
            board_position!(d6, None),
            board_position!(e6, None),
            board_position!(f6, None),
            board_position!(g6, None),
            board_position!(h6, None),
            board_position!(a7, None),
            board_position!(b7, None),
            board_position!(c7, None),
            board_position!(d7, None),
            board_position!(e7, None),
            board_position!(f7, None),
            board_position!(g7, None),
            board_position!(h7, None),
            board_position!(a8, None),
            board_position!(b8, None),
            board_position!(c8, None),
            board_position!(d8, None),
            board_position!(e8, None),
            board_position!(f8, None),
            board_position!(g8, None),
            board_position!(h8, None),
        ];

        for (position, piece) in position_tests {
            assert_eq!(board.get_piece(&position), piece.as_ref());
        }

        assert_eq!(*board.get_current_turn(), Side::White);

        assert_eq!(
            *board.get_castle_rights(),
            CastleRights::new(true, true, true, true)
        );

        assert_eq!(*board.get_en_passant_target(), None);

        assert_eq!(board.get_half_moves(), 0);

        assert_eq!(board.get_full_moves(), 1);
    }
}
