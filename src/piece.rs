#[derive(Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Eq, PartialEq, Clone)]
pub enum Side {
    White = 0,
    Black = 1,
}

#[derive(Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub side: Side,
}

impl Piece {
    pub fn new(piece_type: PieceType, side: Side) -> Piece {
        Piece { piece_type, side }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_piece_notation() {
        assert_eq!(Piece::new(PieceType::Pawn, Side::White).to_string(), "P");
        assert_eq!(Piece::new(PieceType::Pawn, Side::Black).to_string(), "p");

        assert_eq!(Piece::new(PieceType::Knight, Side::White).to_string(), "N");
        assert_eq!(Piece::new(PieceType::Knight, Side::Black).to_string(), "n");

        assert_eq!(Piece::new(PieceType::Bishop, Side::White).to_string(), "B");
        assert_eq!(Piece::new(PieceType::Bishop, Side::Black).to_string(), "b");

        assert_eq!(Piece::new(PieceType::Rook, Side::White).to_string(), "R");
        assert_eq!(Piece::new(PieceType::Rook, Side::Black).to_string(), "r");

        assert_eq!(Piece::new(PieceType::Queen, Side::White).to_string(), "Q");
        assert_eq!(Piece::new(PieceType::Queen, Side::Black).to_string(), "q");

        assert_eq!(Piece::new(PieceType::King, Side::White).to_string(), "K");
        assert_eq!(Piece::new(PieceType::King, Side::Black).to_string(), "k");
    }
}
