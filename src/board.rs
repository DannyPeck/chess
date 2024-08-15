pub mod position;
pub mod rank;
pub mod file;

use crate::Side;
use crate::piece::{Piece, PieceType};
use position::Position;

const BOARD_SIZE: usize = 64;
const EMPTY: BoardPosition = BoardPosition { opt_piece: None };

pub struct BoardPosition {
  opt_piece: Option<Piece>
}

impl BoardPosition {
  pub fn new(opt_piece: &Option<Piece>) -> BoardPosition {
    match opt_piece {
      Some(piece) => BoardPosition::from(piece),
      None => BoardPosition::empty()
    }
  }

  pub fn from(piece: &Piece) -> BoardPosition {
    BoardPosition { opt_piece: Some(piece.clone()) }
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

pub struct Board {
  pub positions: [BoardPosition; BOARD_SIZE]
}

fn side_to_notation(side: &Side) -> String {
  match side {
    Side::White => String::from("w"),
    Side::Black => String::from("b")
  }
}

fn piece_type_to_notation(piece_type: &PieceType) -> String {
  match piece_type {
    PieceType::Pawn => String::from("P"),
    PieceType::Rook => String::from("R"),
    PieceType::Knight => String::from("N"),
    PieceType::Bishop => String::from("B"),
    PieceType::King => String::from("K"),
    PieceType::Queen => String::from("Q"),
  }
}

fn piece_to_notation(piece: &Piece) -> String {
  let side_notation = side_to_notation(&piece.side);
  let piece_type_notation = piece_type_to_notation(&piece.piece_type);

  format!("{}{}", side_notation, piece_type_notation)
}

impl Board {
  pub fn new() -> Board {
    let positions: [BoardPosition; BOARD_SIZE] = [EMPTY; BOARD_SIZE];
    Board { positions }
  }

  pub fn add_piece(&mut self, piece: &Piece, position: &Position) {
    self.positions[position.value()] = BoardPosition::from(piece);
  }

  pub fn add_pieces(&mut self, pieces: &Vec<(Piece, Position)>) {
    for (piece, position) in pieces {
      self.add_piece(piece, position);
    }
  }

  pub fn set_position(&mut self, position: &Position, piece: &Option<Piece>) {
    self.positions[position.value()] = BoardPosition::new(piece);
  }

  pub fn contains_piece(&self, position: &Position) -> bool {
    let board_position = &self.positions[position.value()];
    board_position.get_piece().is_some()
  }

  pub fn contains_enemy_piece(&self, position: &Position, side: &Side) -> bool {
    let board_position = &self.positions[position.value()];
    match &board_position.get_piece() {
      Some(piece) => &piece.side != side,
      None => false
    }
  }

  pub fn print_board(&self) {
    for rank in (rank::ONE..=rank::EIGHT).rev() {
      let mut rank_string = String::new();
      for file in file::A..=file::H {
        let position = Position::from_file_and_rank(file, rank).unwrap();
        let piece_notation = match &self.positions[position.value()].get_piece() {
          Some(piece) => piece_to_notation(piece),
          None => String::from("  ")
        };

        let position_string = format!("[{piece_notation}]");
        rank_string.push_str(&position_string);
      }

      println!("{rank_string}");
    }
  }
}