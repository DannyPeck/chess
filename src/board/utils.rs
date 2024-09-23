use std::collections::{HashMap, HashSet};

use crate::{
    board::position::{Offset, Position},
    piece::{Piece, PieceType, PromotionType, Side},
    ParseError,
};

use super::{file, rank, Board};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MoveState {
    CanMove,
    Stalemate,
    Check,
    Checkmate,
}

#[derive(Debug)]
pub struct MoveError(String);

impl MoveError {
    pub fn new(error: &str) -> MoveError {
        MoveError(String::from(error))
    }
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum MoveKind {
    Move,
    DoubleMove(Position), //  en passant target position
    Capture,
    EnPassant(Position), // capture position
    ShortCastle,
    LongCastle,
    Promotion(bool), // capture
}

#[derive(PartialEq, Eq, Debug)]
pub struct MoveRequest {
    pub start: Position,
    pub end: Position,
    pub promotion: Option<PromotionType>,
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

    pub fn from_coordinate(coordinate_notation: &str) -> Result<MoveRequest, ParseError> {
        if coordinate_notation.len() < 4 {
            return Err(ParseError::new("Notation is incomplete."));
        }

        let start = Position::from_notation(&coordinate_notation[0..2])
            .ok_or(ParseError::new("Invalid start position."))?;
        let end = Position::from_notation(&coordinate_notation[2..4])
            .ok_or(ParseError::new("Invalid end position."))?;
        let promotion = coordinate_notation.chars().nth(4);

        match promotion {
            Some(notation) => match PromotionType::from_coordinate(notation) {
                Some(promotion_type) => Ok(MoveRequest::promotion(start, end, promotion_type)),
                None => Err(ParseError::new("Invalid promotion notation.")),
            },
            None => Ok(MoveRequest::new(start, end)),
        }
    }
}

#[derive(Debug)]
pub struct MoveInfo {
    pub start: Position,
    pub end: Position,
    pub piece_type: PieceType,
    pub is_capture: bool,
    pub file_disambiguation: bool,
    pub rank_disambiguation: bool,
    pub move_kind: MoveKind,
    pub move_state: Option<MoveState>,
    pub promotion: Option<PromotionType>,
}

impl MoveInfo {
    pub fn to_notation(&self) -> String {
        let mut notation = String::new();

        match self.move_kind {
            MoveKind::ShortCastle => {
                notation.push_str("O-O");
            }
            MoveKind::LongCastle => {
                notation.push_str("O-O-O");
            }
            _ => {
                match self.piece_type {
                    PieceType::Pawn => {
                        if self.is_capture {
                            notation.push(file::to_char(self.start.file()));
                        }
                    }
                    PieceType::Knight => {
                        notation.push('N');
                    }
                    PieceType::Bishop => {
                        notation.push('B');
                    }
                    PieceType::Rook => {
                        notation.push('R');
                    }
                    PieceType::Queen => {
                        notation.push('Q');
                    }
                    PieceType::King => {
                        notation.push('K');
                    }
                }

                if self.file_disambiguation {
                    notation.push(file::to_char(self.start.file()));
                }

                if self.rank_disambiguation {
                    notation.push(rank::to_char(self.start.rank()));
                }

                if self.is_capture {
                    notation.push('x');
                }

                let end = format!("{}", self.end);
                notation.push_str(end.as_str());

                if let Some(promotion) = &self.promotion {
                    let promition_notation = format!("={}", promotion.to_algebraic());
                    notation.push_str(promition_notation.as_str());
                }
            }
        }

        if let Some(move_state) = &self.move_state {
            match move_state {
                MoveState::Check => {
                    notation.push('+');
                }
                MoveState::Checkmate => {
                    notation.push('#');
                }
                _ => (),
            }
        }

        notation
    }
}

pub fn move_piece(board: &mut Board, request: MoveRequest) -> Result<MoveInfo, MoveError> {
    let move_kind = get_move(board, &request)?;

    let side = board.get_current_turn();

    // Filter out invalid castles that pass through check
    if move_kind == MoveKind::ShortCastle || move_kind == MoveKind::LongCastle {
        let opponent = side.opponent();
        let opponent_target_positions = get_all_target_positions(board, &opponent);

        let pass_through_check = match (side, &move_kind) {
            (Side::White, MoveKind::ShortCastle) => {
                opponent_target_positions.contains(&Position::f1())
            }
            (Side::White, MoveKind::LongCastle) => {
                opponent_target_positions.contains(&Position::d1())
            }
            (Side::Black, MoveKind::ShortCastle) => {
                opponent_target_positions.contains(&Position::f8())
            }
            (Side::Black, MoveKind::LongCastle) => {
                opponent_target_positions.contains(&Position::d8())
            }
            _ => false,
        };

        if pass_through_check {
            return Err(MoveError::new("Invalid move, cannot move through check."));
        }
    }

    // Always take the piece from the start square.
    let moving_piece = board.take_piece(&request.start).unwrap();

    // Special handling for en passant because the position of the captured piece is not on the end position.
    // Note that this must happen before we update the en passant target.
    if let MoveKind::EnPassant(en_passant_capture) = &move_kind {
        board.set_position(en_passant_capture, None);
    }

    // Set the en passant target
    if let MoveKind::DoubleMove(en_passant_target) = &move_kind {
        board.en_passant_target = Some(en_passant_target.clone());
    } else {
        board.en_passant_target = None;
    }

    // Handle castling
    match (&moving_piece.piece_type, &moving_piece.side) {
        (PieceType::Rook, Side::White) => {
            if request.start == Position::a1() {
                board.castle_rights.white_long_castle_rights = false;
            } else if request.start == Position::h1() {
                board.castle_rights.white_short_castle_rights = false;
            }
        }
        (PieceType::Rook, Side::Black) => {
            if request.start == Position::a8() {
                board.castle_rights.black_long_castle_rights = false;
            } else if request.start == Position::h8() {
                board.castle_rights.black_short_castle_rights = false;
            }
        }
        (PieceType::King, Side::White) => {
            board.castle_rights.white_long_castle_rights = false;
            board.castle_rights.white_short_castle_rights = false;

            match &move_kind {
                MoveKind::ShortCastle => {
                    let rook = board.take_piece(&Position::h1()).unwrap();
                    board.set_position(&Position::f1(), Some(rook));
                }
                MoveKind::LongCastle => {
                    let rook = board.take_piece(&Position::a1()).unwrap();
                    board.set_position(&Position::d1(), Some(rook));
                }
                _ => (),
            }
        }
        (PieceType::King, Side::Black) => {
            board.castle_rights.black_long_castle_rights = false;
            board.castle_rights.black_short_castle_rights = false;

            match &move_kind {
                MoveKind::ShortCastle => {
                    let rook = board.take_piece(&Position::h8()).unwrap();
                    board.set_position(&Position::f8(), Some(rook));
                }
                MoveKind::LongCastle => {
                    let rook = board.take_piece(&Position::a8()).unwrap();
                    board.set_position(&Position::d8(), Some(rook));
                }
                _ => (),
            }
        }
        _ => (),
    }

    // Update the have move counter
    let is_pawn_move = moving_piece.piece_type == PieceType::Pawn;
    let is_capture = matches!(
        move_kind,
        MoveKind::Capture | MoveKind::EnPassant(_) | MoveKind::Promotion(true)
    );

    let reset_half_moves = is_pawn_move || is_capture;
    if reset_half_moves {
        board.half_moves = 0;
    } else {
        board.half_moves += 1;
    }

    let initial_piece_type = moving_piece.piece_type.clone();
    let piece = match move_kind {
        MoveKind::Promotion(_) => {
            // We would not get the MoveKind promotion if it was an invalid request.
            let promotion_piece_type = request.promotion.as_ref().unwrap().to_piece_type();
            Piece::new(promotion_piece_type, board.get_current_turn().clone())
        }
        _ => moving_piece,
    };

    // Place the piece on it's destination square.
    board.set_position(&request.end, Some(piece));

    board.change_turn();

    let move_info = MoveInfo {
        start: request.start,
        end: request.end,
        piece_type: initial_piece_type,
        is_capture,
        file_disambiguation: false,
        rank_disambiguation: false,
        move_kind,
        move_state: None,
        promotion: request.promotion,
    };

    Ok(move_info)
}

pub fn get_move(board: &Board, request: &MoveRequest) -> Result<MoveKind, MoveError> {
    let moves = get_piece_moves(board, board.get_current_turn(), &request.start)?;
    let move_kind = moves
        .get(&request.end)
        .ok_or(MoveError::new("Provided move is not valid."))?;

    if let (MoveKind::Promotion(_), None) = (move_kind, &request.promotion) {
        return Err(MoveError::new(
            "Invalid move request, missing promotion data.",
        ));
    }

    Ok(move_kind.clone())
}

pub fn get_piece_moves(
    board: &Board,
    side: &Side,
    start: &Position,
) -> Result<HashMap<Position, MoveKind>, MoveError> {
    match board.get_piece(start) {
        Some(piece) => {
            if piece.side == *side {
                let moves = match piece.piece_type {
                    PieceType::Pawn => get_pawn_moves(board, start, &piece.side),
                    PieceType::Rook => get_rook_moves(board, start, &piece.side),
                    PieceType::Knight => get_knight_moves(board, start, &piece.side),
                    PieceType::Bishop => get_bishop_moves(board, start, &piece.side),
                    PieceType::King => get_king_moves(board, start, &piece.side),
                    PieceType::Queen => get_queen_moves(board, start, &piece.side),
                };

                Ok(moves)
            } else {
                Err(MoveError::new(
                    "Unable to find a piece for the current player at the provided position.",
                ))
            }
        }
        None => Err(MoveError::new("No piece found at the provided position.")),
    }
}

pub fn get_pawn_moves(board: &Board, start: &Position, side: &Side) -> HashMap<Position, MoveKind> {
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

    if let Some(new_position) = Position::from_offset(start, &forward_one) {
        if !contains_piece(board, &new_position) {
            let move_kind = if new_position.rank() == promotion_rank {
                MoveKind::Promotion(false)
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
        let forward_one_empty = !contains_piece(board, &forward_one);
        let forward_two_empty = !contains_piece(board, &forward_two);

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

        if is_en_passant_target(board, &en_passant_target) {
            Some(en_passant_target)
        } else {
            None
        }
    };

    let diagonal_moves = vec![left_diagonal, right_diagonal];
    for diagonal_move in diagonal_moves {
        if let Some(new_position) = Position::from_offset(start, &diagonal_move) {
            if contains_enemy_piece(board, &new_position, side) {
                let move_kind = if new_position.rank() == promotion_rank {
                    MoveKind::Promotion(true)
                } else {
                    MoveKind::Capture
                };
                valid_positions.insert(new_position, move_kind);
            } else if let Some(en_passant_capture) = en_passant_move(&new_position) {
                valid_positions.insert(new_position, MoveKind::EnPassant(en_passant_capture));
            }
        }
    }

    valid_positions
}

pub fn get_knight_moves(
    board: &Board,
    start: &Position,
    side: &Side,
) -> HashMap<Position, MoveKind> {
    let mut valid_positions = HashMap::new();

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
        if let Some(new_position) = Position::from_offset(start, &offset) {
            if contains_enemy_piece(board, &new_position, side) {
                valid_positions.insert(new_position, MoveKind::Capture);
            } else if !contains_piece(board, &new_position) {
                valid_positions.insert(new_position, MoveKind::Move);
            }
        }
    }

    valid_positions
}

pub fn get_rook_moves(board: &Board, start: &Position, side: &Side) -> HashMap<Position, MoveKind> {
    let offsets = vec![
        Offset::new(1, 0),
        Offset::new(0, 1),
        Offset::new(-1, 0),
        Offset::new(0, -1),
    ];

    get_while_valid(board, start, side, &offsets)
}

pub fn get_bishop_moves(
    board: &Board,
    start: &Position,
    side: &Side,
) -> HashMap<Position, MoveKind> {
    let offsets = vec![
        Offset::new(1, 1),
        Offset::new(-1, 1),
        Offset::new(1, -1),
        Offset::new(-1, -1),
    ];
    get_while_valid(board, start, side, &offsets)
}

pub fn get_queen_moves(
    board: &Board,
    start: &Position,
    side: &Side,
) -> HashMap<Position, MoveKind> {
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
    get_while_valid(board, start, side, &offsets)
}

pub fn get_king_moves(board: &Board, start: &Position, side: &Side) -> HashMap<Position, MoveKind> {
    let mut valid_positions = HashMap::new();

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
        if let Some(new_position) = Position::from_offset(start, &offset) {
            if contains_enemy_piece(board, &new_position, side) {
                valid_positions.insert(new_position, MoveKind::Capture);
            } else if !contains_piece(board, &new_position) {
                valid_positions.insert(new_position, MoveKind::Move);
            }
        }
    }

    // Castling
    match side {
        Side::White => {
            if board.castle_rights.white_short_castle_rights {
                let castle_positions = vec![Position::f1(), Position::g1()];
                if are_positions_empty(board, &castle_positions) {
                    valid_positions.insert(Position::g1(), MoveKind::ShortCastle);
                }
            }

            if board.castle_rights.white_long_castle_rights {
                let castle_positions = vec![Position::b1(), Position::c1(), Position::d1()];
                if are_positions_empty(board, &castle_positions) {
                    valid_positions.insert(Position::c1(), MoveKind::LongCastle);
                }
            }
        }
        Side::Black => {
            if board.castle_rights.black_short_castle_rights {
                let castle_positions = vec![Position::f8(), Position::g8()];
                if are_positions_empty(board, &castle_positions) {
                    valid_positions.insert(Position::g8(), MoveKind::ShortCastle);
                }
            }

            if board.castle_rights.black_long_castle_rights {
                let castle_positions = vec![Position::b8(), Position::c8(), Position::d8()];
                if are_positions_empty(board, &castle_positions) {
                    valid_positions.insert(Position::c8(), MoveKind::LongCastle);
                }
            }
        }
    }

    valid_positions
}

pub fn get_while_valid(
    board: &Board,
    position: &Position,
    side: &Side,
    offsets: &Vec<Offset>,
) -> HashMap<Position, MoveKind> {
    let mut valid_positions = HashMap::new();

    let filter = |new_position: &Position| {
        if !contains_piece(board, new_position) {
            WhileMoveResult::Continue
        } else if contains_enemy_piece(board, new_position, side) {
            WhileMoveResult::Capture
        } else {
            WhileMoveResult::Stop
        }
    };

    for offset in offsets {
        add_while_valid(position, offset, filter, &mut valid_positions);
    }

    valid_positions
}

pub enum WhileMoveResult {
    Continue,
    Capture,
    Stop,
}

pub fn add_while_valid<F>(
    start: &Position,
    offset: &Offset,
    filter: F,
    valid_positions: &mut HashMap<Position, MoveKind>,
) where
    F: Fn(&Position) -> WhileMoveResult,
{
    // Don't allow no-op offsets
    if offset.file_offset == 0 && offset.rank_offset == 0 {
        return;
    }

    let mut current_position = start.clone();
    while let Some(new_position) = Position::from_offset(&current_position, offset) {
        match filter(&new_position) {
            WhileMoveResult::Continue => {
                current_position = new_position.clone();
                valid_positions.insert(new_position, MoveKind::Move);
            }
            WhileMoveResult::Capture => {
                valid_positions.insert(new_position, MoveKind::Capture);
                break;
            }
            WhileMoveResult::Stop => break,
        }
    }
}

pub fn get_all_moves(board: &Board, side: &Side) -> HashMap<Position, HashMap<Position, MoveKind>> {
    let mut all_moves: HashMap<Position, HashMap<Position, MoveKind>> = HashMap::new();

    let piece_positions = match side {
        Side::White => board.get_white_positions(),
        Side::Black => board.get_black_positions(),
    };

    for position in piece_positions {
        if let Ok(moves) = get_piece_moves(board, side, position) {
            all_moves.insert(position.clone(), moves);
        }
    }

    all_moves
}

pub fn get_all_target_positions(board: &Board, side: &Side) -> HashSet<Position> {
    let mut all_target_positions = HashSet::new();

    let piece_positions = match side {
        Side::White => board.get_white_positions(),
        Side::Black => board.get_black_positions(),
    };

    for position in piece_positions {
        if let Ok(moves) = get_piece_moves(board, side, position) {
            all_target_positions.extend(moves.into_keys());
        }
    }

    all_target_positions
}

pub fn is_in_check(board: &Board, side: &Side) -> bool {
    let opponent_side = side.opponent();

    let all_opponent_target_positions = get_all_target_positions(board, &opponent_side);

    for target_position in all_opponent_target_positions {
        if board.get_piece(&target_position) == Some(&Piece::new(PieceType::King, side.clone())) {
            return true;
        }
    }

    false
}

pub fn get_move_state(board: &Board) -> MoveState {
    let all_legal_moves = get_all_legal_moves(board, board.get_current_turn());

    if all_legal_moves.is_empty() {
        if is_in_check(board, board.get_current_turn()) {
            MoveState::Checkmate
        } else {
            MoveState::Stalemate
        }
    } else if board.get_half_moves() == 100 {
        MoveState::Stalemate
    } else if is_in_check(board, board.get_current_turn()) {
        MoveState::Check
    } else {
        MoveState::CanMove
    }
}

pub fn get_all_legal_moves(
    board: &Board,
    side: &Side,
) -> HashMap<Position, HashMap<Position, MoveKind>> {
    let mut all_legal_moves = HashMap::new();
    let all_moves = get_all_moves(board, side);
    for (start, mut piece_moves) in all_moves {
        piece_moves.retain(|end, move_kind| {
            let move_request = match move_kind {
                // Just pick a promotion type, it's just to ensure that the move_piece() call succeeds.
                MoveKind::Promotion(_) => {
                    MoveRequest::promotion(start.clone(), end.clone(), PromotionType::Queen)
                }
                _ => MoveRequest::new(start.clone(), end.clone()),
            };

            let mut new_board = board.clone();
            move_piece(&mut new_board, move_request).is_ok() && !is_in_check(&new_board, side)
        });
        
        if !piece_moves.is_empty() {
            all_legal_moves.insert(start, piece_moves);
        }
    }

    all_legal_moves
}

pub fn contains_piece(board: &Board, position: &Position) -> bool {
    board.get_piece(position).is_some()
}

pub fn contains_enemy_piece(board: &Board, position: &Position, side: &Side) -> bool {
    match board.get_piece(position) {
        Some(piece) => piece.side != *side,
        None => false,
    }
}

pub fn are_positions_empty(board: &Board, positions: &Vec<Position>) -> bool {
    let mut empty = true;
    for position in positions {
        if contains_piece(board, position) {
            empty = false;
            break;
        }
    }

    empty
}

pub fn is_en_passant_target(board: &Board, position: &Position) -> bool {
    match board.get_en_passant_target() {
        Some(en_passant_target) => position == en_passant_target,
        None => false,
    }
}

pub fn possible_en_passant_capture(board: &Board) -> bool {
    match board.get_en_passant_target() {
        Some(target) => {
            let side = board.get_current_turn();
            let left_diagonal = match side {
                Side::White => Position::from_offset(target, &Offset::new(-1, -1)),
                Side::Black => Position::from_offset(target, &Offset::new(-1, 1)),
            };

            let right_diagonal = match side {
                Side::White => Position::from_offset(target, &Offset::new(1, -1)),
                Side::Black => Position::from_offset(target, &Offset::new(-1, -1)),
            };

            let mut valid_capture = false;
            if let Some(left_diagonal) = left_diagonal {
                if let Ok(moves) = get_piece_moves(board, side, &left_diagonal) {
                    valid_capture = moves.contains_key(target);
                };
            };

            // Only check the next position if we didn't already find a valid capture.
            if !valid_capture {
                if let Some(right_diagonal) = right_diagonal {
                    if let Ok(moves) = get_piece_moves(board, side, &right_diagonal) {
                        valid_capture = moves.contains_key(target);
                    };
                }
            }

            valid_capture
        }
        None => false,
    }
}

#[macro_export]
macro_rules! board_position {
    ( $position:ident, None ) => {
        (Position::$position(), None)
    };

    ( $position:ident, $piece_type:ident, $side:ident ) => {
        (
            Position::$position(),
            Some(Piece::new(PieceType::$piece_type, Side::$side)),
        )
    };
}

#[macro_export]
macro_rules! piece_position {
    ( $position:ident, $piece_type:ident, $side:ident ) => {
        (
            Position::$position(),
            Piece::new(PieceType::$piece_type, Side::$side),
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::fen;

    use super::*;

    #[test]
    fn move_request_test() {
        // Normal move request
        {
            let move_request = MoveRequest::new(Position::a2(), Position::a3());
            let expected_move_request = MoveRequest {
                start: Position::a2(),
                end: Position::a3(),
                promotion: None,
            };
            assert_eq!(move_request, expected_move_request);
        }

        // Promotion move request
        {
            let move_request =
                MoveRequest::promotion(Position::a7(), Position::a8(), PromotionType::Queen);
            let expected_move_request = MoveRequest {
                start: Position::a7(),
                end: Position::a8(),
                promotion: Some(PromotionType::Queen),
            };
            assert_eq!(move_request, expected_move_request);
        }
    }

    #[test]
    fn move_request_from_coordinate_test() -> Result<(), ParseError> {
        // Normal move
        {
            let move_request = MoveRequest::from_coordinate("e3e4").unwrap();
            let expected_move_request = MoveRequest::new(Position::e3(), Position::e4());

            assert_eq!(move_request, expected_move_request);
        }

        // Invalid start position
        assert!(MoveRequest::from_coordinate("e9e4").is_err());

        // Invalid end position
        assert!(MoveRequest::from_coordinate("e3x2").is_err());

        // Too small
        assert!(MoveRequest::from_coordinate("e3e").is_err());

        // Queen promotion
        {
            let move_request = MoveRequest::from_coordinate("a7a8q")?;
            let expected_move_request =
                MoveRequest::promotion(Position::a7(), Position::a8(), PromotionType::Queen);

            assert_eq!(move_request, expected_move_request);
        }

        // Knight promotion
        {
            let move_request = MoveRequest::from_coordinate("a7a8n")?;
            let expected_move_request =
                MoveRequest::promotion(Position::a7(), Position::a8(), PromotionType::Knight);

            assert_eq!(move_request, expected_move_request);
        }

        // Bishop promotion
        {
            let move_request = MoveRequest::from_coordinate("a7a8b")?;
            let expected_move_request =
                MoveRequest::promotion(Position::a7(), Position::a8(), PromotionType::Bishop);

            assert_eq!(move_request, expected_move_request);
        }

        // Rook promotion
        {
            let move_request = MoveRequest::from_coordinate("a7a8r")?;
            let expected_move_request =
                MoveRequest::promotion(Position::a7(), Position::a8(), PromotionType::Rook);

            assert_eq!(move_request, expected_move_request);
        }

        // Invalid promotion
        assert!(MoveRequest::from_coordinate("a7a8p").is_err());

        Ok(())
    }

    #[test]
    fn get_pawn_moves_white() -> Result<(), ParseError> {
        // White starting line
        {
            let board = Board::default();
            let moves = get_pawn_moves(&board, &Position::f2(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::f3(), MoveKind::Move),
                (Position::f4(), MoveKind::DoubleMove(Position::f3())),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White single move
        {
            let board = fen::parse("rnbqkbnr/ppp1pppp/3p4/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 2")?;
            let moves = get_pawn_moves(&board, &Position::d3(), &Side::White);
            let expected_moves = HashMap::from([(Position::d4(), MoveKind::Move)]);

            assert_eq!(moves, expected_moves);
        }

        // White right diagonal captures
        {
            let board =
                fen::parse("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 2")?;
            let moves = get_pawn_moves(&board, &Position::d4(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::d5(), MoveKind::Move),
                (Position::e5(), MoveKind::Capture),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White left diagonal captures
        {
            let board =
                fen::parse("rnbqkbnr/pp1ppppp/8/2p5/3P4/8/PPP1PPPP/RNBQKBNR w KQkq c6 0 2")?;
            let moves = get_pawn_moves(&board, &Position::d4(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::d5(), MoveKind::Move),
                (Position::c5(), MoveKind::Capture),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White can't move
        {
            let board =
                fen::parse("rnbqkbnr/pp1ppppp/8/3P4/8/P1p5/1PP1PPPP/RNBQKBNR w KQkq - 0 4")?;
            let moves = get_pawn_moves(&board, &Position::c2(), &Side::White);
            let expected_moves = HashMap::new();

            assert_eq!(moves, expected_moves);
        }

        // White en passant left
        {
            let board =
                fen::parse("rnbqkbnr/1p1ppppp/3P4/p1p5/8/8/PPP1PPPP/RNBQKBNR w KQkq c6 0 4")?;
            let moves = get_pawn_moves(&board, &Position::d6(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::c7(), MoveKind::EnPassant(Position::c6())),
                (Position::e7(), MoveKind::Capture),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White en passant right
        {
            let board =
                fen::parse("rnbqkbnr/pppp1pp1/3P4/4p2p/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 4")?;
            let moves = get_pawn_moves(&board, &Position::d6(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::e7(), MoveKind::EnPassant(Position::e6())),
                (Position::c7(), MoveKind::Capture),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White promotion
        {
            let board =
                fen::parse("rn1qkbnr/ppP1ppp1/3p3p/5b2/8/8/P1PPPPPP/RNBQKBNR w KQkq - 0 5")?;
            let moves = get_pawn_moves(&board, &Position::c7(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::b8(), MoveKind::Promotion(true)),
                (Position::c8(), MoveKind::Promotion(false)),
                (Position::d8(), MoveKind::Promotion(true)),
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
            let moves = get_pawn_moves(&board, &Position::f7(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::f6(), MoveKind::Move),
                (Position::f5(), MoveKind::DoubleMove(Position::f6())),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black single move
        {
            let board = fen::parse("rnbqkbnr/ppp1pppp/3p4/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 2")?;
            let moves = get_pawn_moves(&board, &Position::d6(), &Side::Black);
            let expected_moves = HashMap::from([(Position::d5(), MoveKind::Move)]);

            assert_eq!(moves, expected_moves);
        }

        // Black right diagonal captures
        {
            let board =
                fen::parse("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 2")?;
            let moves = get_pawn_moves(&board, &Position::e5(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::e4(), MoveKind::Move),
                (Position::d4(), MoveKind::Capture),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black left diagonal captures
        {
            let board =
                fen::parse("rnbqkbnr/pp1ppppp/8/2p5/3P4/8/PPP1PPPP/RNBQKBNR w KQkq c6 0 2")?;
            let moves = get_pawn_moves(&board, &Position::c5(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::d4(), MoveKind::Capture),
                (Position::c4(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black can't move
        {
            let board = fen::parse("rnbqkbnr/pp1ppppp/3P4/8/2p5/8/PPP1PPPP/RNBQKBNR b KQkq - 0 3")?;
            let moves = get_pawn_moves(&board, &Position::d7(), &Side::Black);
            let expected_moves = HashMap::new();

            assert_eq!(moves, expected_moves);
        }

        // Black en passant left
        {
            let board =
                fen::parse("rnbqkbnr/ppp1pppp/7P/8/4P3/3p4/PPPP1PP1/RNBQKBNR b KQkq e3 0 4")?;
            let moves = get_pawn_moves(&board, &Position::d3(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::e2(), MoveKind::EnPassant(Position::e3())),
                (Position::c2(), MoveKind::Capture),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black en passant right
        {
            let board =
                fen::parse("rnbqkbnr/ppp1pppp/7P/8/2P5/3p4/PP1PPPP1/RNBQKBNR b KQkq c3 0 4")?;
            let moves = get_pawn_moves(&board, &Position::d3(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::c2(), MoveKind::EnPassant(Position::c3())),
                (Position::e2(), MoveKind::Capture),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black promotion
        {
            let board = fen::parse("rnbqkbnr/p1pppppp/8/6B1/8/3P4/PPp1PPPP/RN1QKBNR b KQkq - 1 5")?;
            let moves = get_pawn_moves(&board, &Position::c2(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::b1(), MoveKind::Promotion(true)),
                (Position::c1(), MoveKind::Promotion(false)),
                (Position::d1(), MoveKind::Promotion(true)),
            ]);

            assert_eq!(moves, expected_moves);
        }

        Ok(())
    }

    #[test]
    fn get_knight_moves_test() -> Result<(), ParseError> {
        // All moves
        {
            let board =
                fen::parse("rnbqkbnr/3ppppp/ppp5/8/4N3/3P1P2/PPP1P1PP/R1BQKBNR b KQkq - 0 4")?;
            let moves = get_knight_moves(&board, &Position::e4(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::f6(), MoveKind::Move),
                (Position::g5(), MoveKind::Move),
                (Position::g3(), MoveKind::Move),
                (Position::f2(), MoveKind::Move),
                (Position::d2(), MoveKind::Move),
                (Position::c3(), MoveKind::Move),
                (Position::c5(), MoveKind::Move),
                (Position::d6(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // No moves
        {
            let board = fen::parse("rnbqkbnr/1ppppppp/p7/8/8/P1P5/1P1PPPPP/RNBQKBNR b KQkq - 0 2")?;
            let moves = get_knight_moves(&board, &Position::b1(), &Side::White);
            let expected_moves = HashMap::new();

            assert_eq!(moves, expected_moves);
        }

        // Left side of board
        {
            let board = fen::parse("rnbqkbnr/2pppppp/pp6/8/8/N1P5/PP1PPPPP/R1BQKBNR w KQkq - 0 3")?;
            let moves = get_knight_moves(&board, &Position::a3(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::b5(), MoveKind::Move),
                (Position::c4(), MoveKind::Move),
                (Position::c2(), MoveKind::Move),
                (Position::b1(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Right side of board
        {
            let board = fen::parse("rnbqkbnr/pppppp2/6pp/8/8/5P1N/PPPPP1PP/RNBQKB1R w KQkq - 0 3")?;
            let moves = get_knight_moves(&board, &Position::h3(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::g5(), MoveKind::Move),
                (Position::f4(), MoveKind::Move),
                (Position::f2(), MoveKind::Move),
                (Position::g1(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Captures and own pieces
        {
            let board =
                fen::parse("rnbqkbnr/p1p1ppp1/1p1p3p/8/4N3/3P4/PPP1PPPP/R1BQKBNR w KQkq - 0 4")?;
            let moves = get_knight_moves(&board, &Position::e4(), &Side::White);
            // No f2 because our piece is there, but still d6 because black's piece is there.
            let expected_moves = HashMap::from([
                (Position::f6(), MoveKind::Move),
                (Position::g5(), MoveKind::Move),
                (Position::g3(), MoveKind::Move),
                (Position::d2(), MoveKind::Move),
                (Position::c3(), MoveKind::Move),
                (Position::c5(), MoveKind::Move),
                (Position::d6(), MoveKind::Capture),
            ]);

            assert_eq!(moves, expected_moves);
        }

        Ok(())
    }

    #[test]
    fn get_rook_moves_test() -> Result<(), ParseError> {
        // All directions empty to edge of board
        {
            let board = fen::parse("r1bqkbnr/3pppp1/P6p/2p5/1R6/2N5/2PPPPPP/2BQKBNR w Kkq - 0 9")?;
            let moves = get_rook_moves(&board, &Position::b4(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::b1(), MoveKind::Move),
                (Position::b2(), MoveKind::Move),
                (Position::b3(), MoveKind::Move),
                (Position::b5(), MoveKind::Move),
                (Position::b6(), MoveKind::Move),
                (Position::b7(), MoveKind::Move),
                (Position::b8(), MoveKind::Move),
                (Position::a4(), MoveKind::Move),
                (Position::c4(), MoveKind::Move),
                (Position::d4(), MoveKind::Move),
                (Position::e4(), MoveKind::Move),
                (Position::f4(), MoveKind::Move),
                (Position::g4(), MoveKind::Move),
                (Position::h4(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Captures & own pieces
        {
            let board =
                fen::parse("r1bqkbnr/3ppp2/P1p3pp/8/2Rn4/1P6/2PPPPPP/1NBQKBNR w Kkq - 0 8")?;
            let moves = get_rook_moves(&board, &Position::c4(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::a4(), MoveKind::Move),
                (Position::b4(), MoveKind::Move),
                (Position::d4(), MoveKind::Capture),
                (Position::c3(), MoveKind::Move),
                (Position::c5(), MoveKind::Move),
                (Position::c6(), MoveKind::Capture),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // No moves
        {
            let board = Board::default();
            let moves = get_rook_moves(&board, &Position::a1(), &Side::White);
            let expected_moves = HashMap::new();

            assert_eq!(moves, expected_moves);
        }

        Ok(())
    }

    #[test]
    fn get_bishop_moves_test() -> Result<(), ParseError> {
        // All directions empty to edge of board
        {
            let board =
                fen::parse("rnbqkbnr/1p2pp1p/p1pp2p1/8/8/3PBP1N/PPP1P1PP/RN1QKB1R w KQkq - 0 5")?;
            let moves = get_bishop_moves(&board, &Position::e3(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::c1(), MoveKind::Move),
                (Position::d2(), MoveKind::Move),
                (Position::f4(), MoveKind::Move),
                (Position::g5(), MoveKind::Move),
                (Position::h6(), MoveKind::Move),
                (Position::a7(), MoveKind::Move),
                (Position::b6(), MoveKind::Move),
                (Position::c5(), MoveKind::Move),
                (Position::d4(), MoveKind::Move),
                (Position::f2(), MoveKind::Move),
                (Position::g1(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Captures & own pieces
        {
            let board =
                fen::parse("rnbqkbnr/1p2ppp1/p2p3p/2p5/8/3PBP2/PPP1PNPP/RN1QKB1R w KQkq - 0 6")?;
            let moves = get_bishop_moves(&board, &Position::e3(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::c1(), MoveKind::Move),
                (Position::d2(), MoveKind::Move),
                (Position::f4(), MoveKind::Move),
                (Position::g5(), MoveKind::Move),
                (Position::h6(), MoveKind::Capture),
                (Position::c5(), MoveKind::Capture),
                (Position::d4(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // No moves
        {
            let board = Board::default();
            let moves = get_bishop_moves(&board, &Position::c1(), &Side::White);
            let expected_moves = HashMap::new();

            assert_eq!(moves, expected_moves);
        }

        Ok(())
    }

    #[test]
    fn get_queen_moves_test() -> Result<(), ParseError> {
        // All directions empty to edge of board
        {
            let board =
                fen::parse("r1b1kbn1/1p3p1r/p1n1p1p1/7p/3Q4/PP3P1N/R1P1P1PP/1NB1KB1R w Kq - 2 12")?;
            let moves = get_queen_moves(&board, &Position::d4(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::a4(), MoveKind::Move),
                (Position::b4(), MoveKind::Move),
                (Position::c4(), MoveKind::Move),
                (Position::e4(), MoveKind::Move),
                (Position::f4(), MoveKind::Move),
                (Position::g4(), MoveKind::Move),
                (Position::h4(), MoveKind::Move),
                (Position::d1(), MoveKind::Move),
                (Position::d2(), MoveKind::Move),
                (Position::d3(), MoveKind::Move),
                (Position::d5(), MoveKind::Move),
                (Position::d6(), MoveKind::Move),
                (Position::d7(), MoveKind::Move),
                (Position::d8(), MoveKind::Move),
                (Position::a7(), MoveKind::Move),
                (Position::b6(), MoveKind::Move),
                (Position::c5(), MoveKind::Move),
                (Position::e3(), MoveKind::Move),
                (Position::f2(), MoveKind::Move),
                (Position::g1(), MoveKind::Move),
                (Position::a1(), MoveKind::Move),
                (Position::b2(), MoveKind::Move),
                (Position::c3(), MoveKind::Move),
                (Position::e5(), MoveKind::Move),
                (Position::f6(), MoveKind::Move),
                (Position::g7(), MoveKind::Move),
                (Position::h8(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Captures & own pieces
        {
            let board =
                fen::parse("r3k1n1/3b1pbr/ppn1p1p1/7p/3Q1P2/PPP3PN/R3P2P/1NB1KB1R w Kq - 1 15")?;
            let moves = get_queen_moves(&board, &Position::d4(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::a4(), MoveKind::Move),
                (Position::b4(), MoveKind::Move),
                (Position::c4(), MoveKind::Move),
                (Position::e4(), MoveKind::Move),
                (Position::d1(), MoveKind::Move),
                (Position::d2(), MoveKind::Move),
                (Position::d3(), MoveKind::Move),
                (Position::d5(), MoveKind::Move),
                (Position::d6(), MoveKind::Move),
                (Position::d7(), MoveKind::Capture),
                (Position::b6(), MoveKind::Capture),
                (Position::c5(), MoveKind::Move),
                (Position::e3(), MoveKind::Move),
                (Position::f2(), MoveKind::Move),
                (Position::g1(), MoveKind::Move),
                (Position::e5(), MoveKind::Move),
                (Position::f6(), MoveKind::Move),
                (Position::g7(), MoveKind::Capture),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // No moves
        {
            let board = Board::default();
            let moves = get_queen_moves(&board, &Position::d1(), &Side::White);
            let expected_moves = HashMap::new();

            assert_eq!(moves, expected_moves);
        }

        Ok(())
    }

    #[test]
    fn get_king_moves_test() -> Result<(), ParseError> {
        // All directions
        {
            let board = fen::parse("rnbqkbnr/2pppppp/4P3/1p6/3K4/p7/PPPP1PPP/RNBQ1BNR w kq - 0 7")?;
            let moves = get_king_moves(&board, &Position::d4(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::d5(), MoveKind::Move),
                (Position::e5(), MoveKind::Move),
                (Position::e4(), MoveKind::Move),
                (Position::e3(), MoveKind::Move),
                (Position::d3(), MoveKind::Move),
                (Position::c3(), MoveKind::Move),
                (Position::c4(), MoveKind::Move),
                (Position::c5(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Captures & own pieces, no checks or castles
        {
            let board = fen::parse("rnbqkbnr/1p1pppp1/p6p/8/2pKP3/8/PPPP1PPP/RNBQ1BNR w kq - 0 5")?;
            let moves = get_king_moves(&board, &Position::d4(), &Side::White);
            // Still c4 as a capture, but not e4 because of our own piece
            let expected_moves = HashMap::from([
                (Position::d5(), MoveKind::Move),
                (Position::e5(), MoveKind::Move),
                (Position::e3(), MoveKind::Move),
                (Position::d3(), MoveKind::Move),
                (Position::c3(), MoveKind::Move),
                (Position::c4(), MoveKind::Capture),
                (Position::c5(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White short & long castles
        {
            let board =
                fen::parse("r3k2r/ppp1pp1p/2nqbnpb/3p4/3P4/2NQBNPB/PPP1PP1P/R3K2R w KQkq - 4 8")?;
            let moves = get_king_moves(&board, &Position::e1(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::d1(), MoveKind::Move),
                (Position::d2(), MoveKind::Move),
                (Position::f1(), MoveKind::Move),
                (Position::c1(), MoveKind::LongCastle),
                (Position::g1(), MoveKind::ShortCastle),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White no long castle because rook move
        {
            let board =
                fen::parse("r3k2r/ppp1ppbp/2nqbnp1/3p4/3P4/2NQBNPB/PPP1PP1P/1R2K2R w Kkq - 6 9")?;
            let moves = get_king_moves(&board, &Position::e1(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::d1(), MoveKind::Move),
                (Position::d2(), MoveKind::Move),
                (Position::f1(), MoveKind::Move),
                (Position::g1(), MoveKind::ShortCastle),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White no short castle because rook move
        {
            let board =
                fen::parse("r3k2r/ppp1ppbp/2nqbnp1/3p4/3P4/2NQBNPB/PPP1PP1P/R3K1R1 w Qkq - 6 9")?;
            let moves = get_king_moves(&board, &Position::e1(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::d1(), MoveKind::Move),
                (Position::d2(), MoveKind::Move),
                (Position::f1(), MoveKind::Move),
                (Position::c1(), MoveKind::LongCastle),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White no castle because king move
        {
            let board =
                fen::parse("r3k2r/ppp1ppbp/2nqbnp1/3p4/3P4/2NQBNPB/PPP1PP1P/R2K3R w kq - 6 9")?;
            let moves = get_king_moves(&board, &Position::d1(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::d2(), MoveKind::Move),
                (Position::c1(), MoveKind::Move),
                (Position::e1(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White no long castle because piece on b1
        {
            let board =
                fen::parse("rn2kbnr/ppp1pppp/3qb3/3p4/3P4/3QB3/PPP1PPPP/RN2KBNR w KQkq - 4 4")?;
            let moves = get_king_moves(&board, &Position::e1(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::d1(), MoveKind::Move),
                (Position::d2(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White no long castle because piece on c1
        {
            let board =
                fen::parse("rnb1kbnr/pp2pppp/2pq4/3p4/3P4/2NQ4/PPP1PPPP/R1B1KBNR w KQkq - 0 4")?;
            let moves = get_king_moves(&board, &Position::e1(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::d1(), MoveKind::Move),
                (Position::d2(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White no long castle because piece on d1
        {
            let board =
                fen::parse("rnbqkbnr/pp3ppp/2p1p3/3p4/3P4/N3B3/PPP1PPPP/R2QKBNR w KQkq - 0 4")?;
            let moves = get_king_moves(&board, &Position::e1(), &Side::White);
            let expected_moves = HashMap::from([(Position::d2(), MoveKind::Move)]);

            assert_eq!(moves, expected_moves);
        }

        // White no short castle because piece on f1
        {
            let board = fen::parse("rnbqkbnr/pppppp1p/6p1/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2")?;
            let moves = get_king_moves(&board, &Position::e1(), &Side::White);
            let expected_moves = HashMap::new();

            assert_eq!(moves, expected_moves);
        }

        // White no short castle because piece on g1
        {
            let board =
                fen::parse("rnbqkbnr/ppp2ppp/3pp3/8/8/3BP3/PPPP1PPP/RNBQK1NR w KQkq - 0 3")?;
            let moves = get_king_moves(&board, &Position::e1(), &Side::White);
            let expected_moves = HashMap::from([
                (Position::e2(), MoveKind::Move),
                (Position::f1(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // White no moves
        {
            let board = Board::default();
            let moves = get_king_moves(&board, &Position::e1(), &Side::White);
            let expected_moves = HashMap::new();

            assert_eq!(moves, expected_moves);
        }

        // Black short & long castles
        {
            let board =
                fen::parse("r3k2r/ppp1pp1p/2nqbnpb/3p4/3P4/2PQPPP1/PP5P/RNB1KBNR b KQkq - 0 8")?;
            let moves = get_king_moves(&board, &Position::e8(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::d8(), MoveKind::Move),
                (Position::d7(), MoveKind::Move),
                (Position::f8(), MoveKind::Move),
                (Position::c8(), MoveKind::LongCastle),
                (Position::g8(), MoveKind::ShortCastle),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black no long castle because rook move
        {
            let board =
                fen::parse("1r2k2r/ppp1pp1p/2nqbnpb/3p4/3P1P2/2PQP1P1/PP5P/RNB1KBNR b KQk - 0 9")?;
            let moves = get_king_moves(&board, &Position::e8(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::d8(), MoveKind::Move),
                (Position::d7(), MoveKind::Move),
                (Position::f8(), MoveKind::Move),
                (Position::g8(), MoveKind::ShortCastle),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black no short castle because rook move
        {
            let board =
                fen::parse("r3k1r1/ppp1pp1p/2nqbnpb/3p4/3P2P1/2PQPP2/PP5P/RNB1KBNR b KQq - 0 9")?;
            let moves = get_king_moves(&board, &Position::e8(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::d8(), MoveKind::Move),
                (Position::d7(), MoveKind::Move),
                (Position::f8(), MoveKind::Move),
                (Position::c8(), MoveKind::LongCastle),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black no castle because king move
        {
            let board =
                fen::parse("r2k3r/ppp1pp1p/2nqbnpb/3p4/3P2P1/2PQPP2/PP5P/RNB1KBNR b KQ - 0 9")?;
            let moves = get_king_moves(&board, &Position::d8(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::d7(), MoveKind::Move),
                (Position::c8(), MoveKind::Move),
                (Position::e8(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black no long castle because piece on b8
        {
            let board =
                fen::parse("rn2kbnr/ppp1pppp/3qb3/3p4/3P4/2P5/PP1QPPPP/RNB1KBNR b KQkq - 0 4")?;
            let moves = get_king_moves(&board, &Position::e8(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::d8(), MoveKind::Move),
                (Position::d7(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black no long castle because piece on c8
        {
            let board =
                fen::parse("r1b1kbnr/ppp1pppp/2nq4/3p4/3P4/2P1P3/PP3PPP/RNBQKBNR b KQkq - 0 4")?;
            let moves = get_king_moves(&board, &Position::e8(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::d8(), MoveKind::Move),
                (Position::d7(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black no long castle because piece on d8
        {
            let board =
                fen::parse("r2qkbnr/ppp1pppp/2n5/3p1b2/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 4")?;
            let moves = get_king_moves(&board, &Position::e8(), &Side::Black);
            let expected_moves = HashMap::from([(Position::d7(), MoveKind::Move)]);

            assert_eq!(moves, expected_moves);
        }

        // Black no short castle because piece on f8
        {
            let board =
                fen::parse("rnbqkb1r/pppppppp/7n/8/8/2N2P2/PPPPP1PP/R1BQKBNR b KQkq - 0 2")?;
            let moves = get_king_moves(&board, &Position::e8(), &Side::Black);
            let expected_moves = HashMap::new();

            assert_eq!(moves, expected_moves);
        }

        // Black no short castle because piece on g8
        {
            let board =
                fen::parse("rnbqk1nr/pppp1ppp/3bp3/8/8/3PPP2/PPP3PP/RNBQKBNR b KQkq - 0 3")?;
            let moves = get_king_moves(&board, &Position::e8(), &Side::Black);
            let expected_moves = HashMap::from([
                (Position::e7(), MoveKind::Move),
                (Position::f8(), MoveKind::Move),
            ]);

            assert_eq!(moves, expected_moves);
        }

        // Black no moves
        {
            let board = Board::default();
            let moves = get_king_moves(&board, &Position::e8(), &Side::Black);
            let expected_moves = HashMap::new();

            assert_eq!(moves, expected_moves);
        }

        Ok(())
    }

    #[test]
    fn get_all_moves_test() -> Result<(), ParseError> {
        let board =
            fen::parse("r3k1n1/3b1pbr/ppn1p1p1/7p/3Q1P2/PPP3PN/R3P2P/1NB1KB1R w Kq - 1 15")?;

        let all_white_moves = get_all_moves(&board, &Side::White);

        let expected_white_moves = HashMap::from([
            (
                Position::a3(),
                HashMap::from([(Position::a4(), MoveKind::Move)]),
            ),
            (
                Position::b3(),
                HashMap::from([(Position::b4(), MoveKind::Move)]),
            ),
            (
                Position::c3(),
                HashMap::from([(Position::c4(), MoveKind::Move)]),
            ),
            (
                Position::e2(),
                HashMap::from([
                    (Position::e3(), MoveKind::Move),
                    (Position::e4(), MoveKind::DoubleMove(Position::e3())),
                ]),
            ),
            (
                Position::f4(),
                HashMap::from([(Position::f5(), MoveKind::Move)]),
            ),
            (
                Position::g3(),
                HashMap::from([(Position::g4(), MoveKind::Move)]),
            ),
            (Position::h2(), HashMap::from([])),
            (
                Position::a2(),
                HashMap::from([
                    (Position::a1(), MoveKind::Move),
                    (Position::b2(), MoveKind::Move),
                    (Position::c2(), MoveKind::Move),
                    (Position::d2(), MoveKind::Move),
                ]),
            ),
            (
                Position::b1(),
                HashMap::from([(Position::d2(), MoveKind::Move)]),
            ),
            (
                Position::c1(),
                HashMap::from([
                    (Position::b2(), MoveKind::Move),
                    (Position::d2(), MoveKind::Move),
                    (Position::e3(), MoveKind::Move),
                ]),
            ),
            (
                Position::f1(),
                HashMap::from([(Position::g2(), MoveKind::Move)]),
            ),
            (
                Position::h1(),
                HashMap::from([(Position::g1(), MoveKind::Move)]),
            ),
            (
                Position::h3(),
                HashMap::from([
                    (Position::g5(), MoveKind::Move),
                    (Position::g1(), MoveKind::Move),
                    (Position::f2(), MoveKind::Move),
                ]),
            ),
            (
                Position::e1(),
                HashMap::from([
                    (Position::d1(), MoveKind::Move),
                    (Position::d2(), MoveKind::Move),
                    (Position::f2(), MoveKind::Move),
                ]),
            ),
            (
                Position::d4(),
                HashMap::from([
                    (Position::a4(), MoveKind::Move),
                    (Position::b4(), MoveKind::Move),
                    (Position::c4(), MoveKind::Move),
                    (Position::e4(), MoveKind::Move),
                    (Position::d1(), MoveKind::Move),
                    (Position::d2(), MoveKind::Move),
                    (Position::d3(), MoveKind::Move),
                    (Position::d5(), MoveKind::Move),
                    (Position::d6(), MoveKind::Move),
                    (Position::d7(), MoveKind::Capture),
                    (Position::b6(), MoveKind::Capture),
                    (Position::c5(), MoveKind::Move),
                    (Position::e3(), MoveKind::Move),
                    (Position::f2(), MoveKind::Move),
                    (Position::g1(), MoveKind::Move),
                    (Position::e5(), MoveKind::Move),
                    (Position::f6(), MoveKind::Move),
                    (Position::g7(), MoveKind::Capture),
                ]),
            ),
        ]);

        assert_eq!(all_white_moves, expected_white_moves);

        let all_black_moves = get_all_moves(&board, &Side::Black);

        let expected_black_moves = HashMap::from([
            (
                Position::a6(),
                HashMap::from([(Position::a5(), MoveKind::Move)]),
            ),
            (
                Position::b6(),
                HashMap::from([(Position::b5(), MoveKind::Move)]),
            ),
            (
                Position::e6(),
                HashMap::from([(Position::e5(), MoveKind::Move)]),
            ),
            (
                Position::f7(),
                HashMap::from([
                    (Position::f6(), MoveKind::Move),
                    (Position::f5(), MoveKind::DoubleMove(Position::f6())),
                ]),
            ),
            (
                Position::g6(),
                HashMap::from([(Position::g5(), MoveKind::Move)]),
            ),
            (
                Position::h5(),
                HashMap::from([(Position::h4(), MoveKind::Move)]),
            ),
            (
                Position::a8(),
                HashMap::from([
                    (Position::a7(), MoveKind::Move),
                    (Position::b8(), MoveKind::Move),
                    (Position::c8(), MoveKind::Move),
                    (Position::d8(), MoveKind::Move),
                ]),
            ),
            (
                Position::c6(),
                HashMap::from([
                    (Position::a7(), MoveKind::Move),
                    (Position::b8(), MoveKind::Move),
                    (Position::d8(), MoveKind::Move),
                    (Position::e7(), MoveKind::Move),
                    (Position::e5(), MoveKind::Move),
                    (Position::d4(), MoveKind::Capture),
                    (Position::b4(), MoveKind::Move),
                    (Position::a5(), MoveKind::Move),
                ]),
            ),
            (
                Position::d7(),
                HashMap::from([(Position::c8(), MoveKind::Move)]),
            ),
            (
                Position::g8(),
                HashMap::from([
                    (Position::e7(), MoveKind::Move),
                    (Position::f6(), MoveKind::Move),
                    (Position::h6(), MoveKind::Move),
                ]),
            ),
            (
                Position::g7(),
                HashMap::from([
                    (Position::f8(), MoveKind::Move),
                    (Position::h8(), MoveKind::Move),
                    (Position::h6(), MoveKind::Move),
                    (Position::f6(), MoveKind::Move),
                    (Position::e5(), MoveKind::Move),
                    (Position::d4(), MoveKind::Capture),
                ]),
            ),
            (
                Position::h7(),
                HashMap::from([
                    (Position::h8(), MoveKind::Move),
                    (Position::h6(), MoveKind::Move),
                ]),
            ),
            (
                Position::e8(),
                HashMap::from([
                    (Position::f8(), MoveKind::Move),
                    (Position::e7(), MoveKind::Move),
                    (Position::d8(), MoveKind::Move),
                    (Position::c8(), MoveKind::LongCastle),
                ]),
            ),
        ]);

        assert_eq!(all_black_moves, expected_black_moves);

        Ok(())
    }

    #[test]
    fn is_in_check_test() -> Result<(), ParseError> {
        // White in check
        {
            let board =
                fen::parse("rnb1kbnr/pp1ppppp/8/q1p5/8/3P1P2/PPP1P1PP/RNBQKBNR w KQkq - 1 3")?;

            assert!(is_in_check(&board, &Side::White));
        }

        // White not in check
        {
            let board =
                fen::parse("rnbqkbnr/pp1ppppp/8/2p5/8/3P1P2/PPP1P1PP/RNBQKBNR b KQkq - 0 2")?;

            assert!(!is_in_check(&board, &Side::Black));
        }

        // Black in check
        {
            let board =
                fen::parse("rnbqkbnr/ppppp2p/8/5ppQ/5P2/4P3/PPPP2PP/RNB1KBNR b KQkq - 1 3")?;

            assert!(is_in_check(&board, &Side::Black));
        }

        // Black not in check
        {
            let board =
                fen::parse("rnbqkbnr/ppppp2p/8/5pp1/5P2/4P3/PPPP2PP/RNBQKBNR w KQkq g6 0 3")?;

            assert!(!is_in_check(&board, &Side::Black));
        }

        Ok(())
    }

    #[test]
    fn get_move_state_test() -> Result<(), ParseError> {
        // White in checkmate
        {
            let board =
                fen::parse("rnb1kbnr/pppp1ppp/4p3/8/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3")?;

            assert_eq!(get_move_state(&board), MoveState::Checkmate);
        }

        // White in check
        {
            let board =
                fen::parse("rnb1kbnr/pppp1ppp/4p3/8/7q/3P1P2/PPP1P1PP/RNBQKBNR w KQkq - 1 3")?;

            assert_eq!(get_move_state(&board), MoveState::Check);
        }

        // White in stalemate
        {
            let board = fen::parse("rnb1kbnr/ppp1ppp1/8/8/8/8/4q3/6K1 w kq - 0 1")?;

            assert_eq!(get_move_state(&board), MoveState::Stalemate);
        }

        // White in 50 move rule stalemate
        {
            let board =
                fen::parse("rnb1kbnr/ppppqppp/4p3/8/8/3P1P2/PPP1P1PP/RNBQKBNR w KQkq - 100 50")?;

            assert_eq!(get_move_state(&board), MoveState::Stalemate);
        }

        // White not in check
        {
            let board =
                fen::parse("rnb1kbnr/ppppqppp/4p3/8/8/3P1P2/PPP1P1PP/RNBQKBNR w KQkq - 1 3")?;

            assert_eq!(get_move_state(&board), MoveState::CanMove);
        }

        // Black in checkmate
        {
            let board =
                fen::parse("rnbqkbnr/ppppp2p/5p2/6pQ/5P2/4P3/PPPP2PP/RNB1KBNR b KQkq - 1 3")?;

            assert_eq!(get_move_state(&board), MoveState::Checkmate);
        }

        // Black in check
        {
            let board =
                fen::parse("rnbqkbnr/ppp1p1pp/3p1p2/7Q/5P2/4P3/PPPP2PP/RNB1KBNR b KQkq - 1 3")?;

            assert_eq!(get_move_state(&board), MoveState::Check);
        }

        // Black in stalemate
        {
            let board = fen::parse("1R6/8/8/8/p2R4/k7/8/1K6 b - - 0 99")?;

            assert_eq!(get_move_state(&board), MoveState::Stalemate);
        }

        // Black in 50 move stalemate
        {
            let board =
                fen::parse("rnbqkbnr/ppp1p1pp/3p1p2/8/5P2/4PQ2/PPPP2PP/RNB1KBNR b KQkq - 100 50")?;

            assert_eq!(get_move_state(&board), MoveState::Stalemate);
        }

        // Black not in check
        {
            let board =
                fen::parse("rnbqkbnr/ppp1p1pp/3p1p2/8/5P2/4PQ2/PPPP2PP/RNB1KBNR b KQkq - 1 3")?;

            assert_eq!(get_move_state(&board), MoveState::CanMove);
        }

        Ok(())
    }

    #[test]
    fn get_all_legal_moves_test() -> Result<(), ParseError> {
        {
            let board =
                fen::parse("rnbqkbnr/pp1pp1pp/2p2p2/7Q/5P2/4P3/PPPP2PP/RNB1KBNR b KQkq - 1 3")?;

            let all_legal_moves = get_all_legal_moves(&board, board.get_current_turn());

            let expected_legal_moves = HashMap::from([(
                Position::g7(),
                HashMap::from([(Position::g6(), MoveKind::Move)]),
            )]);

            assert_eq!(all_legal_moves, expected_legal_moves);
        }

        // White no long castle because passthrough check
        {
            let board =
                fen::parse("rn2kbnr/ppp2ppp/1q1pp3/8/2B1P1b1/NP6/PBPP1PPP/R3K1NR w KQkq - 0 7")?;

            let all_legal_moves = get_all_legal_moves(&board, board.get_current_turn());

            // Note that long castling is not a legal move, even though white still
            // has long castle rights and the start & end positions are not targets.
            // It is not legal because the king passes through check on d1.
            let king_moves = all_legal_moves.get(&Position::e1()).unwrap();
            assert!(!king_moves.contains_key(&Position::c1()));
            assert!(board.get_castle_rights().white_long_castle_rights);
        }

        // White no short castle because passthrough check
        {
            let board =
                fen::parse("rn2kbnr/ppp1ppp1/3p3p/8/2q1P1b1/NP3P1N/PBPP2PP/R3K2R w KQkq - 0 9")?;

            let all_legal_moves = get_all_legal_moves(&board, board.get_current_turn());

            // Note that long castling is not a legal move, even though white still
            // has long castle rights and the start & end positions are not targets.
            // It is not legal because the king passes through check on d1.
            let king_moves = all_legal_moves.get(&Position::e1()).unwrap();
            assert!(!king_moves.contains_key(&Position::g1()));
            assert!(board.get_castle_rights().white_short_castle_rights);
        }

        // Black no long castle because passthrough check
        {
            let board = fen::parse("r3kbn1/pp2pppr/n2Q3p/P1P5/8/2P4P/P3PP1P/RNB1KBNR b KQq - 0 8")?;

            let all_legal_moves = get_all_legal_moves(&board, board.get_current_turn());

            // The king has no valid moves.
            // Note that long castling is not a legal move, even though black still
            // has long castle rights and the start & end positions are not targets.
            // It is not legal because the king passes through check on d8.
            assert!(!all_legal_moves.contains_key(&Position::e8()));

            assert!(board.get_castle_rights().black_long_castle_rights);
        }

        // Black no short castle because passthrough check
        {
            let board =
                fen::parse("rnb1k2r/ppqp1ppp/2p4n/4p3/1Q6/b1PP2PP/PP2PP2/RNB1KBNR b KQkq - 0 6")?;

            let all_legal_moves = get_all_legal_moves(&board, board.get_current_turn());

            // Note that long castling is not a legal move, even though black still
            // has long castle rights and the start & end positions are not targets.
            // It is not legal because the king passes through check on d8.
            let king_moves = all_legal_moves.get(&Position::e8()).unwrap();
            assert!(!king_moves.contains_key(&Position::g8()));
            assert!(board.get_castle_rights().black_short_castle_rights);
        }

        Ok(())
    }
}
