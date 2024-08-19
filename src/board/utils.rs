use std::collections::HashSet;

use crate::board::{Position, PositionOffset};

pub fn get_if_valid<F>(position: &Position, offset: PositionOffset, filter: F) -> Option<Position>
where
    F: Fn(&Position) -> bool,
{
    Position::from_offset(&position, offset).filter(filter)
}

pub fn add_while_valid<F>(
    start_position: &Position,
    file_offset: i32,
    rank_offset: i32,
    filter: F,
    valid_positions: &mut HashSet<Position>,
) where
    F: Fn(&Position) -> bool,
{
    let file = start_position.file();
    let rank = start_position.rank();

    let mut current_file = file as i32;
    let mut current_rank = rank as i32;
    loop {
        current_file += file_offset;
        current_rank += rank_offset;

        match get_if_valid(
            start_position,
            PositionOffset::new(current_file, current_rank),
            &filter,
        ) {
            Some(new_position) => valid_positions.insert(new_position),
            None => break,
        };
    }
}

pub fn positions_to_string(positions: &HashSet<Position>) -> String {
    let mut output = String::new();

    let mut counter = 0;
    for position in positions {
        if counter > 0 {
            output += ", ";
        }

        output += position.to_string().as_str();

        counter += 1;
    }

    output
}
