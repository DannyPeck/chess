use crate::board::{position::Position, Board};

pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn new(board: Board) -> Game {
        Game { board }
    }

    pub fn attempt_move(&mut self, start_position: &Position, end_position: &Position) -> bool {
        self.board.move_piece(start_position, end_position)
    }
}
