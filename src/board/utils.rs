use std::collections::HashMap;

use crate::board::position::{Offset, Position};

use super::MoveKind;

pub fn get_if_valid<F>(position: &Position, offset: &Offset, filter: F) -> Option<Position>
where
    F: Fn(&Position) -> bool,
{
    Position::from_offset(&position, offset).filter(filter)
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
    loop {
        match Position::from_offset(&current_position, offset) {
            Some(new_position) => match filter(&new_position) {
                WhileMoveResult::Continue => {
                    current_position = new_position.clone();
                    valid_positions.insert(new_position, MoveKind::Move);
                }
                WhileMoveResult::Capture => {
                    valid_positions.insert(new_position, MoveKind::Capture);
                    break;
                }
                WhileMoveResult::Stop => break,
            },
            None => break,
        };
    }
}

pub fn positions_to_string(positions: &HashMap<Position, MoveKind>) -> String {
    let mut output = String::new();

    let mut counter = 0;
    for (position, _) in positions {
        if counter > 0 {
            output += ", ";
        }

        output += position.to_string().as_str();

        counter += 1;
    }

    output
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
