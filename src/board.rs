pub mod file;
pub mod position;
pub mod rank;
pub mod utils;

use std::collections::HashMap;

use crate::piece::{Piece, PieceType, PromotionType, Side};
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
        pieces: Vec<(Piece, Position)>,
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

    pub fn get_piece(&self, position: &Position) -> Option<&Piece> {
        self.positions[position.value()].as_ref()
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

    pub fn add_piece(&mut self, piece: Piece, position: &Position) {
        self.set_position(&position, Some(piece));
    }

    pub fn add_pieces(&mut self, pieces: Vec<(Piece, Position)>) {
        for (piece, position) in pieces {
            self.add_piece(piece, &position);
        }
    }

    fn take_piece(&mut self, position: &Position) -> Option<Piece> {
        self.positions[position.value()].take()
    }

    fn determine_en_passant_target(&self, start: &Position, side: &Side) -> Option<Position> {
        match side {
            Side::White => Position::from_offset(start, &Offset::new(0, 1)),
            Side::Black => Position::from_offset(start, &Offset::new(0, -1)),
        }
    }

    fn determine_en_passant_capture(&self, end: &Position, side: &Side) -> Option<Position> {
        match side {
            Side::White => Position::from_offset(end, &Offset::new(0, -1)),
            Side::Black => Position::from_offset(end, &Offset::new(0, 1)),
        }
    }

    fn get_move(&self, request: &MoveRequest) -> Result<MoveKind, MoveError> {
        let piece = self
            .get_piece(&request.start)
            .filter(|piece| piece.side == self.current_turn)
            .ok_or(MoveError::new(
                "Unable to find a piece for the current player at the provided position.",
            ))?;

        let moves = self.get_moves(piece, &request.start);
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

        let forward_two = match side {
            Side::White => Offset::new(0, 2),
            Side::Black => Offset::new(0, -2),
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

        let forward_move = |new_position: &Position| !self.contains_piece(new_position);
        let capture_move = |new_position: &Position| self.contains_enemy_piece(new_position, side);
        let en_passant_move = |new_position: &Position| self.is_en_passant_target(new_position);

        if let Some(new_position) = utils::get_if_valid(start, &forward_one, forward_move) {
            let move_kind = if new_position.rank() == promotion_rank {
                MoveKind::Promotion
            } else {
                MoveKind::Move
            };
            valid_positions.insert(new_position, move_kind);
        }

        if let Some(new_position) = utils::get_if_valid(start, &forward_two, forward_move) {
            let en_passant_target = self.determine_en_passant_target(start, side).unwrap();
            valid_positions.insert(new_position, MoveKind::DoubleMove(en_passant_target));
        }

        if let Some(new_position) = utils::get_if_valid(start, &left_diagonal, capture_move) {
            let move_kind = if new_position.rank() == promotion_rank {
                MoveKind::Promotion
            } else {
                MoveKind::Move
            };
            valid_positions.insert(new_position, move_kind);
        }

        if let Some(new_position) = utils::get_if_valid(start, &left_diagonal, en_passant_move) {
            let en_passant_capture = self
                .determine_en_passant_capture(&new_position, side)
                .unwrap();
            valid_positions.insert(new_position, MoveKind::EnPassant(en_passant_capture));
        }

        if let Some(new_position) = utils::get_if_valid(start, &right_diagonal, capture_move) {
            let move_kind = if new_position.rank() == promotion_rank {
                MoveKind::Promotion
            } else {
                MoveKind::Move
            };
            valid_positions.insert(new_position, move_kind);
        }

        if let Some(new_position) = utils::get_if_valid(start, &right_diagonal, en_passant_move) {
            let en_passant_capture = self
                .determine_en_passant_capture(&new_position, side)
                .unwrap();
            valid_positions.insert(new_position, MoveKind::EnPassant(en_passant_capture));
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
        let mut pieces = Vec::new();

        // White piece
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::a2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::b2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::c2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::d2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::e2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::f2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::g2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::h2()));
        pieces.push((Piece::new(PieceType::Rook, Side::White), Position::a1()));
        pieces.push((Piece::new(PieceType::Rook, Side::White), Position::h1()));
        pieces.push((Piece::new(PieceType::Knight, Side::White), Position::b1()));
        pieces.push((Piece::new(PieceType::Knight, Side::White), Position::g1()));
        pieces.push((Piece::new(PieceType::Bishop, Side::White), Position::c1()));
        pieces.push((Piece::new(PieceType::Bishop, Side::White), Position::f1()));
        pieces.push((Piece::new(PieceType::King, Side::White), Position::e1()));
        pieces.push((Piece::new(PieceType::Queen, Side::White), Position::d1()));

        // Black pieces
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::a7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::b7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::c7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::d7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::e7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::f7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::g7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::h7()));
        pieces.push((Piece::new(PieceType::Rook, Side::Black), Position::a8()));
        pieces.push((Piece::new(PieceType::Rook, Side::Black), Position::h8()));
        pieces.push((Piece::new(PieceType::Knight, Side::Black), Position::b8()));
        pieces.push((Piece::new(PieceType::Knight, Side::Black), Position::g8()));
        pieces.push((Piece::new(PieceType::Bishop, Side::Black), Position::c8()));
        pieces.push((Piece::new(PieceType::Bishop, Side::Black), Position::f8()));
        pieces.push((Piece::new(PieceType::King, Side::Black), Position::e8()));
        pieces.push((Piece::new(PieceType::Queen, Side::Black), Position::d8()));

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
