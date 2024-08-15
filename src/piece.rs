use crate::Side;

#[derive(Clone)]
pub enum PieceType {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King
}

#[derive(Clone)]
pub struct Piece {
  pub piece_type: PieceType,
  pub side: Side
}

impl Piece {
  pub fn new(piece_type: PieceType, side: Side) -> Piece {
    Piece {
      piece_type,
      side
    }
  }
}

impl std::fmt::Display for Piece {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut notation = match self.piece_type {
      PieceType::Pawn => String::from("P"),
      PieceType::Rook => String::from("R"),
      PieceType::Knight => String::from("N"),
      PieceType::Bishop => String::from("B"),
      PieceType::King => String::from("K"),
      PieceType::Queen => String::from("Q"),
    };

    if self.side == Side::Black {
      notation = notation.to_ascii_lowercase();
    }

    write!(f, "{notation}")
  }
}