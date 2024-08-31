use std::collections::HashMap;

use crate::board::position::{Offset, Position};

use super::MoveKind;

pub fn get_if_valid<F>(position: &Position, offset: &Offset, filter: F) -> Option<Position>
where
    F: Fn(&Position) -> bool,
{
    Position::from_offset(&position, offset).filter(filter)
}

pub fn add_while_valid<F>(
    start: &Position,
    offset: &Offset,
    filter: F,
    valid_positions: &mut HashMap<Position, MoveKind>,
) where
    F: Fn(&Position) -> bool,
{
    // Don't allow no-op offsets
    if offset.file_offset == 0 && offset.rank_offset == 0 {
        return;
    }

    let mut current_position = start.clone();
    loop {
        match get_if_valid(&current_position, &offset, &filter) {
            Some(new_position) => {
                current_position = new_position.clone();

                valid_positions.insert(new_position, MoveKind::Move);
            }
            None => break,
        };
    }
}

pub fn positions_to_string(positions: &HashMap<Position, MoveKind>) -> String {
    let mut output = String::new();

    let mut counter = 0;
    for (position, move_type) in positions {
        if counter > 0 {
            output += ", ";
        }

        output += position.to_string().as_str();

        counter += 1;
    }

    output
}
