pub mod file;
pub mod position;
pub mod rank;
pub mod utils;

use std::collections::HashMap;

use crate::{
    piece::{Piece, PieceType, PromotionType, Side},
    piece_position,
};
use position::{Offset, Position};

const BOARD_SIZE: usize = 64;
const EMPTY: Option<Piece> = None;

#[derive(Eq, PartialEq, Debug)]
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

#[derive(Debug)]
pub struct MoveError(String);

impl MoveError {
    pub fn new(error: &str) -> MoveError {
        MoveError(String::from(error))
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum MoveKind {
    Move,
    DoubleMove(Position), //  en passant target position
    EnPassant(Position),  // capture position
    ShortCastle,
    LongCastle,
    Promotion,
}

pub struct MoveRequest {
    start: Position,
    end: Position,
    promotion: Option<PromotionType>,
}

impl MoveRequest {
    pub fn new(start: Position, end: Position) -> MoveRequest {
        MoveRequest {
            start,
            end,
            promotion: None,
        }
    }

    pub fn promotion(start: Position, end: Position, promotion_type: PromotionType) -> MoveRequest {
        MoveRequest {
            start,
            end,
            promotion: Some(promotion_type),
        }
    }
}

#[derive(Debug)]
pub struct Board {
    positions: [Option<Piece>; BOARD_SIZE],
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

    fn change_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Side::White => Side::Black,
            Side::Black => {
                // TODO: Update half moves once we have check detection.
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

    pub fn get_piece(&self, position: &Position) -> &Option<Piece> {
        &self.positions[position.value()]
    }

    pub fn is_occupiable_position(&self, position: &Position, side: &Side) -> bool {
        match self.get_piece(position) {
            Some(piece) => piece.side != *side,
            None => true,
        }
    }

    pub fn contains_piece(&self, position: &Position) -> bool {
        self.get_piece(position).is_some()
    }

    pub fn contains_enemy_piece(&self, position: &Position, side: &Side) -> bool {
        match self.get_piece(position) {
            Some(piece) => piece.side != *side,
            None => false,
        }
    }

    pub fn is_en_passant_target(&self, position: &Position) -> bool {
        match &self.en_passant_target {
            Some(en_passant_target) => position == en_passant_target,
            None => false,
        }
    }

    fn set_position(&mut self, position: &Position, opt_piece: Option<Piece>) {
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

    fn take_piece(&mut self, position: &Position) -> Option<Piece> {
        self.positions[position.value()].take()
    }

    fn get_move(&self, request: &MoveRequest) -> Result<MoveKind, MoveError> {
        let piece = self
            .get_piece(&request.start)
            .as_ref()
            .filter(|piece| piece.side == self.current_turn)
            .ok_or(MoveError::new(
                "Unable to find a piece for the current player at the provided position.",
            ))?;

        let moves = self.get_moves(&piece, &request.start);
        let move_kind = moves
            .get(&request.end)
            .ok_or(MoveError::new("Provided move is not valid."))?;

        if *move_kind == MoveKind::Promotion && request.promotion == None {
            return Err(MoveError::new(
                "Invalid move request, missing promotion data.",
            ));
        }

        Ok(move_kind.clone())
    }

    pub fn move_piece(&mut self, request: MoveRequest) -> Result<(), MoveError> {
        let move_kind = self.get_move(&request)?;

        // Always take the piece from the start square.
        let moving_piece = self.take_piece(&request.start).unwrap();

        // Special handling for en passant because the position of the captured piece is not on the end position.
        // Note that this must happen before we update the en passant target.
        if let MoveKind::EnPassant(en_passant_capture) = &move_kind {
            self.set_position(&en_passant_capture, None);
        }

        // Set the en passant target
        if let MoveKind::DoubleMove(en_passant_target) = &move_kind {
            self.en_passant_target = Some(en_passant_target.clone());
        } else {
            self.en_passant_target = None;
        }

        // Handle castling
        match (&moving_piece.piece_type, &moving_piece.side) {
            (PieceType::Rook, Side::White) => {
                if request.start == Position::a1() {
                    self.castle_rights.white_long_castle_rights = false;
                } else if request.start == Position::h1() {
                    self.castle_rights.white_short_castle_rights = false;
                }
            }
            (PieceType::Rook, Side::Black) => {
                if request.start == Position::a8() {
                    self.castle_rights.black_long_castle_rights = false;
                } else if request.start == Position::h8() {
                    self.castle_rights.black_short_castle_rights = false;
                }
            }
            (PieceType::King, Side::White) => {
                self.castle_rights.white_long_castle_rights = false;
                self.castle_rights.white_short_castle_rights = false;

                match &move_kind {
                    MoveKind::ShortCastle => {
                        let rook = self.take_piece(&Position::h1()).unwrap();
                        self.set_position(&Position::f1(), Some(rook));
                    }
                    MoveKind::LongCastle => {
                        let rook = self.take_piece(&Position::a1()).unwrap();
                        self.set_position(&Position::d1(), Some(rook));
                    }
                    _ => (),
                }
            }
            (PieceType::King, Side::Black) => {
                self.castle_rights.black_long_castle_rights = false;
                self.castle_rights.black_short_castle_rights = false;

                match &move_kind {
                    MoveKind::ShortCastle => {
                        let rook = self.take_piece(&Position::h8()).unwrap();
                        self.set_position(&Position::f8(), Some(rook));
                    }
                    MoveKind::LongCastle => {
                        let rook = self.take_piece(&Position::a8()).unwrap();
                        self.set_position(&Position::d8(), Some(rook));
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        let piece = if move_kind == MoveKind::Promotion {
            // We would not get the MoveKind promotion if it was an invalid request.
            let promotion_piece_type = request.promotion.unwrap().to_piece_type();
            Piece::new(promotion_piece_type, self.current_turn.clone())
        } else {
            moving_piece
        };

        // Place the piece on it's destination square.
        self.set_position(&request.end, Some(piece));

        self.change_turn();

        Ok(())
    }

    pub fn get_moves(&self, piece: &Piece, start: &Position) -> HashMap<Position, MoveKind> {
        match piece.piece_type {
            PieceType::Pawn => self.get_pawn_moves(start, &piece.side),
            PieceType::Rook => self.get_rook_moves(start, &piece.side),
            PieceType::Knight => self.get_knight_moves(start, &piece.side),
            PieceType::Bishop => self.get_bishop_moves(start, &piece.side),
            PieceType::King => self.get_king_moves(start, &piece.side),
            PieceType::Queen => self.get_queen_moves(start, &piece.side),
        }
    }

    pub fn get_pawn_moves(&self, start: &Position, side: &Side) -> HashMap<Position, MoveKind> {
        let mut valid_positions = HashMap::new();

        let forward_one = match side {
            Side::White => Offset::new(0, 1),
            Side::Black => Offset::new(0, -1),
        };

        let left_diagonal = match side {
            Side::White => Offset::new(-1, 1),
            Side::Black => Offset::new(1, -1),
        };

        let right_diagonal = match side {
            Side::White => Offset::new(1, 1),
            Side::Black => Offset::new(-1, -1),
        };

        let promotion_rank = match side {
            Side::White => rank::EIGHT,
            Side::Black => rank::ONE,
        };

        if let Some(new_position) = Position::from_offset(&start, &forward_one) {
            if !self.contains_piece(&new_position) {
                let move_kind = if new_position.rank() == promotion_rank {
                    MoveKind::Promotion
                } else {
                    MoveKind::Move
                };
                valid_positions.insert(new_position, move_kind);
            }
        }

        let double_move_positions = match side {
            Side::White if start.rank() == rank::TWO => {
                let forward_one = Position::from_file_and_rank(start.file(), start.rank() + 1);
                let forward_two = Position::from_file_and_rank(start.file(), start.rank() + 2);
                Some((forward_one, forward_two))
            }
            Side::Black if start.rank() == rank::SEVEN => {
                let forward_one = Position::from_file_and_rank(start.file(), start.rank() - 1);
                let forward_two = Position::from_file_and_rank(start.file(), start.rank() - 2);
                Some((forward_one, forward_two))
            }
            _ => None,
        };

        if let Some((forward_one, forward_two)) = double_move_positions {
            let forward_one_empty = !self.contains_piece(&forward_one);
            let forward_two_empty = !self.contains_piece(&forward_two);

            if forward_one_empty && forward_two_empty {
                valid_positions.insert(forward_two, MoveKind::DoubleMove(forward_one));
            }
        }

        let en_passant_move = |new_position: &Position| {
            let en_passant_target = match side {
                Side::White => {
                    Position::from_file_and_rank(new_position.file(), new_position.rank() - 1)
                }
                Side::Black => {
                    Position::from_file_and_rank(new_position.file(), new_position.rank() + 1)
                }
            };
            
            if self.is_en_passant_target(&en_passant_target) {
                Some(en_passant_target)
            } else {
                None
            }
        };

        let diagonal_moves = vec![left_diagonal, right_diagonal];
        for diagonal_move in diagonal_moves {
            if let Some(new_position) = Position::from_offset(&start, &diagonal_move) {
                if self.contains_enemy_piece(&new_position, side) {
                    let move_kind = if new_position.rank() == promotion_rank {
                        MoveKind::Promotion
                    } else {
                        MoveKind::Move
                    };
                    valid_positions.insert(new_position, move_kind);
                } else if let Some(en_passant_capture) = en_passant_move(&new_position) {
                    valid_positions.insert(new_position, MoveKind::EnPassant(en_passant_capture));
                }
            }
        }

        return valid_positions;
    }

    pub fn get_rook_moves(&self, start: &Position, side: &Side) -> HashMap<Position, MoveKind> {
        self.get_linear_moves(start, side)
    }

    pub fn get_knight_moves(&self, start: &Position, side: &Side) -> HashMap<Position, MoveKind> {
        let mut valid_positions = HashMap::new();

        let filter = |new_position: &Position| self.is_occupiable_position(new_position, side);

        let offsets = vec![
            // North East
            Offset::new(1, 2),
            Offset::new(2, 1),
            // South East
            Offset::new(1, -2),
            Offset::new(2, -1),
            // North West
            Offset::new(-1, 2),
            Offset::new(-2, 1),
            // South West
            Offset::new(-2, -1),
            Offset::new(-1, -2),
        ];

        for offset in offsets {
            if let Some(new_position) = utils::get_if_valid(&start, &offset, filter) {
                valid_positions.insert(new_position, MoveKind::Move);
            }
        }

        return valid_positions;
    }

    pub fn get_bishop_moves(&self, start: &Position, side: &Side) -> HashMap<Position, MoveKind> {
        self.get_diagonal_moves(start, side)
    }

    pub fn get_queen_moves(&self, start: &Position, side: &Side) -> HashMap<Position, MoveKind> {
        let mut moves = self.get_linear_moves(start, side);

        let diagonal_moves = self.get_diagonal_moves(start, side);
        moves.extend(diagonal_moves);

        moves
    }

    pub fn get_king_moves(&self, start: &Position, side: &Side) -> HashMap<Position, MoveKind> {
        let mut valid_positions = HashMap::new();

        let filter = |new_position: &Position| self.is_occupiable_position(new_position, side);

        // Regular moves
        let offsets = vec![
            Offset::new(1, 0),
            Offset::new(0, 1),
            Offset::new(-1, 0),
            Offset::new(0, -1),
            Offset::new(1, 1),
            Offset::new(-1, 1),
            Offset::new(1, -1),
            Offset::new(-1, -1),
        ];

        for offset in offsets {
            if let Some(new_position) = utils::get_if_valid(&start, &offset, filter) {
                valid_positions.insert(new_position, MoveKind::Move);
            }
        }

        // Castling
        match self.current_turn {
            Side::White => {
                if self.castle_rights.white_short_castle_rights {
                    valid_positions.insert(Position::g1(), MoveKind::ShortCastle);
                }

                if self.castle_rights.white_long_castle_rights {
                    valid_positions.insert(Position::c1(), MoveKind::LongCastle);
                }
            }
            Side::Black => {
                if self.castle_rights.black_short_castle_rights {
                    valid_positions.insert(Position::g8(), MoveKind::ShortCastle);
                }

                if self.castle_rights.black_long_castle_rights {
                    valid_positions.insert(Position::c8(), MoveKind::LongCastle);
                }
            }
        }

        return valid_positions;
    }

    pub fn get_diagonal_moves(
        &self,
        position: &Position,
        side: &Side,
    ) -> HashMap<Position, MoveKind> {
        let mut valid_positions = HashMap::new();

        let filter = |new_position: &Position| self.is_occupiable_position(new_position, side);

        // Right & Up
        utils::add_while_valid(position, &Offset::new(1, 1), filter, &mut valid_positions);

        // Left & Up
        utils::add_while_valid(position, &Offset::new(-1, 1), filter, &mut valid_positions);

        // Right & Down
        utils::add_while_valid(position, &Offset::new(1, -1), filter, &mut valid_positions);

        // Left & Down
        utils::add_while_valid(position, &Offset::new(-1, -1), filter, &mut valid_positions);

        valid_positions
    }

    pub fn get_linear_moves(
        &self,
        position: &Position,
        side: &Side,
    ) -> HashMap<Position, MoveKind> {
        let mut valid_positions = HashMap::new();

        let filter = |new_position: &Position| self.is_occupiable_position(new_position, side);

        // Up
        utils::add_while_valid(position, &Offset::new(0, 1), filter, &mut valid_positions);

        // Down
        utils::add_while_valid(position, &Offset::new(0, -1), filter, &mut valid_positions);

        // Right
        utils::add_while_valid(position, &Offset::new(1, 0), filter, &mut valid_positions);

        // Left
        utils::add_while_valid(position, &Offset::new(-1, 0), filter, &mut valid_positions);

        valid_positions
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
    use crate::{
        board_position,
        fen::{self, ParseError},
    };

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
            assert_eq!(board.get_piece(&position), &piece);
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
            assert_eq!(board.get_piece(&position), &piece);
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
    fn get_pawn_moves_white() -> Result<(), ParseError> {
        // White starting line
        {
            let board = Board::default();
            let moves = board.get_pawn_moves(&Position::f2(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::f3(), MoveKind::Move),
                (Position::f4(), MoveKind::DoubleMove(Position::f3())),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White single move
        {
            let board = fen::parse_fen("rnbqkbnr/ppp1pppp/3p4/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 2")?;
            let moves = board.get_pawn_moves(&Position::d3(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::d4(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White right diagonal captures
        {
            let board = fen::parse_fen("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 2")?;
            let moves = board.get_pawn_moves(&Position::d4(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::d5(), MoveKind::Move),
                (Position::e5(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White left diagonal captures
        {
            let board = fen::parse_fen("rnbqkbnr/pp1ppppp/8/2p5/3P4/8/PPP1PPPP/RNBQKBNR w KQkq c6 0 2")?;
            let moves = board.get_pawn_moves(&Position::d4(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::d5(), MoveKind::Move),
                (Position::c5(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White can't move
        {
            let board = fen::parse_fen("rnbqkbnr/pp1ppppp/8/3P4/8/P1p5/1PP1PPPP/RNBQKBNR w KQkq - 0 4")?;
            let moves = board.get_pawn_moves(&Position::c2(), &Side::White);
            let expected_moves = HashMap::new();

            assert_eq!(moves, expected_moves);
        }

        // White en passant left
        {
            let board = fen::parse_fen("rnbqkbnr/1p1ppppp/3P4/p1p5/8/8/PPP1PPPP/RNBQKBNR w KQkq c6 0 4")?;
            let moves = board.get_pawn_moves(&Position::d6(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::c7(), MoveKind::EnPassant(Position::c6())),
                (Position::e7(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White en passant right
        {
            let board = fen::parse_fen("rnbqkbnr/pppp1pp1/3P4/4p2p/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 4")?;
            let moves = board.get_pawn_moves(&Position::d6(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::e7(), MoveKind::EnPassant(Position::e6())),
                (Position::c7(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White promotion
        {
            let board = fen::parse_fen("rn1qkbnr/ppP1ppp1/3p3p/5b2/8/8/P1PPPPPP/RNBQKBNR w KQkq - 0 5")?;
            let moves = board.get_pawn_moves(&Position::c7(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::b8(), MoveKind::Promotion),
                (Position::c8(), MoveKind::Promotion),
                (Position::d8(), MoveKind::Promotion),
            ]);

            assert_eq!(moves, expected_moves);
        }

        Ok(())
    }

    #[test]
    fn get_pawn_moves_black() -> Result<(), ParseError> {
        // Black starting line
        {
            let board = Board::default();
            let moves = board.get_pawn_moves(&Position::f7(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::f6(), MoveKind::Move),
                (Position::f5(), MoveKind::DoubleMove(Position::f6())),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black single move
        {
            let board = fen::parse_fen("rnbqkbnr/ppp1pppp/3p4/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 2")?;
            let moves = board.get_pawn_moves(&Position::d6(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::d5(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black right diagonal captures
        {
            let board = fen::parse_fen("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 2")?;
            let moves = board.get_pawn_moves(&Position::e5(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::e4(), MoveKind::Move),
                (Position::d4(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black left diagonal captures
        {
            let board = fen::parse_fen("rnbqkbnr/pp1ppppp/8/2p5/3P4/8/PPP1PPPP/RNBQKBNR w KQkq c6 0 2")?;
            let moves = board.get_pawn_moves(&Position::c5(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::d4(), MoveKind::Move),
                (Position::c4(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black can't move
        {
            let board = fen::parse_fen("rnbqkbnr/pp1ppppp/3P4/8/2p5/8/PPP1PPPP/RNBQKBNR b KQkq - 0 3")?;
            let moves = board.get_pawn_moves(&Position::d7(), &Side::Black);
            let expected_moves = HashMap::new();

            assert_eq!(moves, expected_moves);
        }

        // Black en passant left
        {
            let board = fen::parse_fen("rnbqkbnr/ppp1pppp/7P/8/4P3/3p4/PPPP1PP1/RNBQKBNR b KQkq e3 0 4")?;
            let moves = board.get_pawn_moves(&Position::d3(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::e2(), MoveKind::EnPassant(Position::e3())),
                (Position::c2(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black en passant right
        {
            let board = fen::parse_fen("rnbqkbnr/ppp1pppp/7P/8/2P5/3p4/PP1PPPP1/RNBQKBNR b KQkq c3 0 4")?;
            let moves = board.get_pawn_moves(&Position::d3(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::c2(), MoveKind::EnPassant(Position::c3())),
                (Position::e2(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black promotion
        {
            let board = fen::parse_fen("rnbqkbnr/p1pppppp/8/6B1/8/3P4/PPp1PPPP/RN1QKBNR b KQkq - 1 5")?;
            let moves = board.get_pawn_moves(&Position::c2(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::b1(), MoveKind::Promotion),
                (Position::c1(), MoveKind::Promotion),
                (Position::d1(), MoveKind::Promotion),
            ]);

            assert_eq!(moves, expected_moves);
        }

        Ok(())
    }
}
