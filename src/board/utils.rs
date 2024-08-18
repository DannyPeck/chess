use std::collections::HashSet;

use crate::board::{
    file,
    position::{Position, PositionOffset},
    rank, Board,
};
use crate::piece::{Piece, PieceType};
use crate::Side;

fn is_occupiable_position(board: &Board, side: &Side, position: &Position) -> bool {
    match board.positions[position.value()].get_piece() {
        Some(piece) => side != &piece.side,
        None => true,
    }
}

pub fn get_moves(board: &Board, piece: &Piece, position: &Position) -> HashSet<Position> {
    let legal_board_positions = match piece.piece_type {
        PieceType::Pawn => get_pawn_moves(board, &piece.side, position),
        PieceType::Rook => get_rook_moves(board, &piece.side, position),
        PieceType::Knight => get_pawn_moves(board, &piece.side, position),
        PieceType::Bishop => get_pawn_moves(board, &piece.side, position),
        PieceType::King => get_pawn_moves(board, &piece.side, position),
        PieceType::Queen => get_pawn_moves(board, &piece.side, position),
    };

    legal_board_positions
}

fn get_pawn_moves(board: &Board, side: &Side, position: &Position) -> HashSet<Position> {
    let mut legal_positions = HashSet::new();

    let front_offset = if *side == Side::White {
        PositionOffset::new(0, 1)
    } else {
        PositionOffset::new(0, -1)
    };

    let left_diagonal_offset = if *side == Side::White {
        PositionOffset::new(-1, 1)
    } else {
        PositionOffset::new(1, -1)
    };

    let right_diagonal_offset = if *side == Side::White {
        PositionOffset::new(1, 1)
    } else {
        PositionOffset::new(-1, -1)
    };

    let opt_front = Position::from_offset(&position, front_offset);
    let opt_left_diagonal = Position::from_offset(&position, left_diagonal_offset);
    let opt_right_diagonal = Position::from_offset(&position, right_diagonal_offset);

    if let Some(front) = opt_front {
        if !contains_piece(board, &front) {
            legal_positions.insert(front);
        }
    }

    if let Some(left_diagonal) = opt_left_diagonal {
        if contains_enemy_piece(board, &left_diagonal, &side) {
            legal_positions.insert(left_diagonal);
        }
    }

    if let Some(right_diagonal) = opt_right_diagonal {
        if contains_enemy_piece(board, &right_diagonal, &side) {
            legal_positions.insert(right_diagonal);
        }
    }

    return legal_positions;
}

fn moves_to_string(moves: &HashSet<Position>) -> String {
    let mut output = String::new();

    let mut counter = 0;
    for current_move in moves {
        if counter > 0 {
            output += ", ";
        }
        output += format!("{current_move}").as_str();

        counter = counter + 1;
    }

    output
}

fn get_linear_moves(board: &Board, side: &Side, position: &Position) -> HashSet<Position> {
    let mut legal_positions = HashSet::new();

    let current_rank = position.rank();
    let current_file = position.file();

    for i in current_rank..rank::EIGHT {
        let next_rank = i + 1;
        let next_position = Position::from_file_and_rank(current_file, next_rank);

        if !is_occupiable_position(board, side, &next_position) {
            break;
        }

        legal_positions.insert(next_position);
    }

    for previous_rank in (rank::ONE..current_rank).rev() {
        let next_position = Position::from_file_and_rank(current_file, previous_rank);

        if !is_occupiable_position(board, side, &next_position) {
            break;
        }

        legal_positions.insert(next_position);
    }

    for i in current_file..file::H {
        let next_file = i + 1;
        let next_position = Position::from_file_and_rank(next_file, current_rank);

        if !is_occupiable_position(board, side, &next_position) {
            break;
        }

        legal_positions.insert(next_position);
    }

    for previous_file in (file::A..current_file).rev() {
        let next_position = Position::from_file_and_rank(previous_file, current_rank);

        if !is_occupiable_position(board, side, &next_position) {
            break;
        }

        legal_positions.insert(next_position);
    }

    legal_positions
}

fn get_rook_moves(board: &Board, side: &Side, position: &Position) -> HashSet<Position> {
    get_linear_moves(board, side, position)
}

pub fn move_piece(board: &mut Board, start: &Position, end: &Position) {
    if valid_move(board, start, end) {
        let start_position = &mut board.positions[start.value()];
        let opt_moving_piece = start_position.take_piece();

        let end_position = &mut board.positions[end.value()];
        end_position.set(opt_moving_piece);
    }
}

fn valid_move(board: &Board, start: &Position, end: &Position) -> bool {
    let start_position = &board.positions[start.value()];
    match &start_position.get_piece() {
        Some(piece) => {
            let legal_moves = get_moves(board, piece, start);
            legal_moves.contains(end)
        }
        None => false,
    }
}

pub fn contains_piece(board: &Board, position: &Position) -> bool {
    let board_position = &board.positions[position.value()];
    board_position.get_piece().is_some()
}

pub fn contains_enemy_piece(board: &Board, position: &Position, side: &Side) -> bool {
    let board_position = &board.positions[position.value()];
    match &board_position.get_piece() {
        Some(piece) => &piece.side != side,
        None => false,
    }
}
