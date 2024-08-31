use crate::board::{Board, MoveError, MoveRequest};

pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn new(board: Board) -> Game {
        Game { board }
    }

    pub fn attempt_move(&mut self, request: MoveRequest) -> Result<(), MoveError> {
        self.board.move_piece(request)
    }
}
