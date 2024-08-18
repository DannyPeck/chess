use crate::board::{Board, BoardPosition};

pub struct Game {
  pub board: Board,
  pub whites_turn: bool,
  pub white_short_castle_rights: bool,
  pub white_long_castle_rights: bool,
  pub black_short_castle_rights: bool,
  pub black_long_castle_rights: bool,
  pub en_passant_target: Option<BoardPosition>
}

impl Game {
  pub fn new(board: Board) -> Game {
    Game {
      board,
      whites_turn: true,
      white_short_castle_rights: true,
      white_long_castle_rights: true,
      black_short_castle_rights: true,
      black_long_castle_rights: true,
      en_passant_target: None
    }
  }
}