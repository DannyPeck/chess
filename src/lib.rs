pub mod board;
pub mod piece;
pub mod player;

use std::collections::HashSet;

use board::{position::{Position, PositionOffset}, rank, file, Board};
use piece::{Piece, PieceType};

#[derive(Eq, PartialEq, Clone)]
pub enum Side {
  White = 0,
  Black = 1
}

pub struct Game {
  pub board: Board,
  whites_turn: bool
}

impl Game {
  pub fn new() -> Game {
    let mut board = Board::new();

    let white_pieces = player::white_pieces();
    let black_pieces = player::black_pieces();

    board.add_pieces(&white_pieces);
    board.add_pieces(&black_pieces);

    Game {
      board,
      whites_turn: true
    }
  }

  pub fn get_moves(&self, piece: &Piece, position: &Position) -> HashSet<Position> {
    let legal_board_positions = match piece.piece_type {
      PieceType::Pawn => self.get_pawn_moves(&piece.side, position),
      PieceType::Rook => self.get_rook_moves(&piece.side, position),
      PieceType::Knight => self.get_pawn_moves(&piece.side, position),
      PieceType::Bishop => self.get_pawn_moves(&piece.side, position),
      PieceType::King => self.get_pawn_moves(&piece.side, position),
      PieceType::Queen => self.get_pawn_moves(&piece.side, position)
    };

    legal_board_positions
  }

  fn get_pawn_moves(&self, side: &Side, position: &Position) -> HashSet<Position> {
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
      if !self.board.contains_piece(&front) {
        legal_positions.insert(front);
      }
    }

    if let Some(left_diagonal) = opt_left_diagonal {
      if self.board.contains_enemy_piece(&left_diagonal, &side) {
        legal_positions.insert(left_diagonal);
      }
    }

    if let Some(right_diagonal) = opt_right_diagonal {
      if self.board.contains_enemy_piece(&right_diagonal, &side) {
        legal_positions.insert(right_diagonal);
      }
    }

    return legal_positions;
  }

  // Legal move
  // occupiable square

  fn is_occupiable_position(&self, side: &Side, position: &Position) -> bool {
    match self.board.positions[position.value()].get_piece() {
      Some(piece) => {
        side != &piece.side
      },
      None => true
    }
  }

  fn get_linear_moves(&self, side: &Side, position: &Position) -> HashSet<Position> {
    let mut legal_positions = HashSet::new();

    let current_rank = position.rank();
    let current_file = position.file();

    for i in current_rank..rank::EIGHT {
      let next_rank = i + 1;
      let next_position = Position::from_valid_file_and_rank(current_file, next_rank);
      if self.is_occupiable_position(side, &next_position) {
        legal_positions.insert(next_position);
      } else {
        break;
      }
    }

    for previous_rank in (rank::ONE..current_rank).rev() {
      let next_position = Position::from_valid_file_and_rank(current_file, previous_rank);
      if self.is_occupiable_position(side, &next_position) {
        legal_positions.insert(next_position);
      } else {
        break;
      }
    }

    for i in current_file..file::H {
      let next_file = i + 1;
      let next_position = Position::from_valid_file_and_rank(next_file, current_rank);
      if self.is_occupiable_position(side, &next_position) {
        legal_positions.insert(next_position);
      } else {
        break;
      }
    }

    for previous_file in (file::A..current_file).rev() {
      let next_position = Position::from_valid_file_and_rank(previous_file, current_rank);
      if self.is_occupiable_position(side, &next_position) {
        legal_positions.insert(next_position);
      } else {
        break;
      }
    }

    legal_positions
  }

  fn get_rook_moves(&self, side: &Side, position: &Position) -> HashSet<Position> {
    self.get_linear_moves(side, position)
  }

  fn valid_move(&self, start: &Position, end: &Position) -> bool {
    let start_position = &self.board.positions[start.value()];
    match &start_position.get_piece() {
      Some(piece) => {
        let legal_moves = self.get_moves(piece, start);
        legal_moves.contains(end)
      },
      None => false
    }
  }

  pub fn move_piece(&mut self, start: &Position, end: &Position) {
    if self.valid_move(start, end) {
      let start_position = &mut self.board.positions[start.value()];
      let opt_moving_piece = start_position.take_piece();

      let end_position = &mut self.board.positions[end.value()];
      end_position.set(opt_moving_piece);
    }
  }
}