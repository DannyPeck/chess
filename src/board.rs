pub mod file;
pub mod position;
pub mod rank;
pub mod utils;

use std::collections::HashSet;

use crate::piece::{Piece, PieceType, Side};
use position::{Offset, Position};

const BOARD_SIZE: usize = 64;
const EMPTY: BoardPosition = BoardPosition { opt_piece: None };

#[derive(Debug)]
struct BoardPosition {
    opt_piece: Option<Piece>,
}

impl BoardPosition {
    pub fn new(opt_piece: Option<Piece>) -> BoardPosition {
        BoardPosition { opt_piece }
    }

    pub fn from(piece: Piece) -> BoardPosition {
        BoardPosition {
            opt_piece: Some(piece),
        }
    }

    pub fn empty() -> BoardPosition {
        EMPTY
    }

    pub fn get_piece(&self) -> &Option<Piece> {
        &self.opt_piece
    }

    pub fn set(&mut self, opt_piece: Option<Piece>) {
        self.opt_piece = opt_piece;
    }

    pub fn take_piece(&mut self) -> Option<Piece> {
        let opt_piece = self.opt_piece.clone();

        self.opt_piece = None;

        opt_piece
    }
}

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
pub struct Board {
    positions: [BoardPosition; BOARD_SIZE],
    current_turn: Side,
    castle_rights: CastleRights,
    en_passant_target: Option<Position>,
    half_moves: u32,
    full_moves: u32,
}

impl Board {
    pub fn empty() -> Board {
        let positions: [BoardPosition; BOARD_SIZE] = [EMPTY; BOARD_SIZE];
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
        let positions: [BoardPosition; BOARD_SIZE] = [EMPTY; BOARD_SIZE];
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

    pub fn default() -> Board {
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

    pub fn get_current_turn(&self) -> Side {
        self.current_turn.clone()
    }

    fn change_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Side::White => Side::Black,
            Side::Black => {
                self.full_moves += 1;
                Side::White
            },
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

    fn get_board_position(&self, position: &Position) -> &BoardPosition {
        &self.positions[position.value()]
    }

    pub fn get_piece(&self, position: &Position) -> &Option<Piece> {
        &self.get_board_position(position).opt_piece
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

    pub fn add_piece(&mut self, piece: Piece, position: Position) {
        self.positions[position.value()] = BoardPosition::from(piece);
    }

    pub fn add_pieces(&mut self, pieces: Vec<(Piece, Position)>) {
        for (piece, position) in pieces {
            self.add_piece(piece, position);
        }
    }

    pub fn move_piece(&mut self, start: &Position, end: &Position) -> bool {
        let valid_move = self.valid_move(start, end);
        if valid_move {
            let start_position = &mut self.positions[start.value()];
            let opt_moving_piece = start_position.take_piece();

            let end_position = &mut self.positions[end.value()];
            end_position.set(opt_moving_piece);

            self.change_turn();
        }

        valid_move
    }

    pub fn get_moves(&self, piece: &Piece, position: &Position) -> HashSet<Position> {
        let valid_positions = match piece.piece_type {
            PieceType::Pawn => self.get_pawn_moves(position, &piece.side),
            PieceType::Rook => self.get_rook_moves(position, &piece.side),
            PieceType::Knight => self.get_knight_moves(position, &piece.side),
            PieceType::Bishop => self.get_bishop_moves(position, &piece.side),
            PieceType::King => self.get_king_moves(position, &piece.side),
            PieceType::Queen => self.get_queen_moves(position, &piece.side),
        };

        valid_positions
    }

    pub fn get_pawn_moves(&self, position: &Position, side: &Side) -> HashSet<Position> {
        let mut valid_positions = HashSet::new();

        let forward_one_offset = if *side == Side::White {
            Offset::new(0, 1)
        } else {
            Offset::new(0, -1)
        };

        let forward_two_offset = if *side == Side::White {
            Offset::new(0, 2)
        } else {
            Offset::new(0, -2)
        };

        let left_diagonal_offset = if *side == Side::White {
            Offset::new(-1, 1)
        } else {
            Offset::new(1, -1)
        };

        let right_diagonal_offset = if *side == Side::White {
            Offset::new(1, 1)
        } else {
            Offset::new(-1, -1)
        };

        let forward_move = |new_position: &Position| !self.contains_piece(new_position);
        let diagonal_move = |new_position: &Position| self.contains_enemy_piece(new_position, side);

        if let Some(new_position) =
            utils::get_if_valid(&position, &forward_one_offset, forward_move)
        {
            valid_positions.insert(new_position);
        }

        if let Some(new_position) =
            utils::get_if_valid(&position, &forward_two_offset, forward_move)
        {
            valid_positions.insert(new_position);
        }

        if let Some(new_position) =
            utils::get_if_valid(&position, &left_diagonal_offset, diagonal_move)
        {
            valid_positions.insert(new_position);
        }

        if let Some(new_position) =
            utils::get_if_valid(&position, &right_diagonal_offset, diagonal_move)
        {
            valid_positions.insert(new_position);
        }

        return valid_positions;
    }

    pub fn get_rook_moves(&self, position: &Position, side: &Side) -> HashSet<Position> {
        self.get_linear_moves(position, side)
    }

    pub fn get_knight_moves(&self, position: &Position, side: &Side) -> HashSet<Position> {
        let mut valid_positions = HashSet::new();

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
            if let Some(new_position) = utils::get_if_valid(&position, &offset, filter) {
                valid_positions.insert(new_position);
            }
        }

        return valid_positions;
    }

    pub fn get_bishop_moves(&self, position: &Position, side: &Side) -> HashSet<Position> {
        self.get_diagonal_moves(position, side)
    }

    pub fn get_queen_moves(&self, position: &Position, side: &Side) -> HashSet<Position> {
        let mut moves = self.get_linear_moves(position, side);

        let diagonal_moves = self.get_diagonal_moves(position, side);
        moves.extend(diagonal_moves);

        moves
    }

    pub fn get_king_moves(&self, position: &Position, side: &Side) -> HashSet<Position> {
        let mut valid_positions = HashSet::new();

        let filter = |new_position: &Position| self.is_occupiable_position(new_position, side);

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
            if let Some(new_position) = utils::get_if_valid(&position, &offset, filter) {
                valid_positions.insert(new_position);
            }
        }

        return valid_positions;
    }

    pub fn get_diagonal_moves(&self, position: &Position, side: &Side) -> HashSet<Position> {
        let mut valid_positions = HashSet::new();

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

    pub fn get_linear_moves(&self, position: &Position, side: &Side) -> HashSet<Position> {
        let mut valid_positions = HashSet::new();

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

    pub fn valid_move(&self, start: &Position, end: &Position) -> bool {
        match self.get_board_position(start).get_piece() {
            Some(piece) if piece.side == self.current_turn => {
                let valid_moves = self.get_moves(piece, start);
                valid_moves.contains(end)
            }
            _ => false,
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_string = String::new();
        for rank in (rank::ONE..=rank::EIGHT).rev() {
            let mut rank_string = String::new();
            for file in file::A..=file::H {
                let position = Position::from_file_and_rank(file, rank);
                let piece_notation = match self.positions[position.value()].get_piece() {
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
