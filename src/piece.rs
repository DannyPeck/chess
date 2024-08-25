#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Side {
    White = 0,
    Black = 1,
}

impl Side {
    pub fn from(side: &str) -> Option<Side> {
        match side {
            "w" => Some(Side::White),
            "b" => Some(Side::Black),
            _ => None,
        }
    }
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notation = match self {
            Side::White => String::from("w"),
            Side::Black => String::from("b"),
        };

        write!(f, "{notation}")
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub side: Side,
}

impl Piece {
    pub fn new(piece_type: PieceType, side: Side) -> Piece {
        Piece { piece_type, side }
    }

    pub fn from(notation: char) -> Option<Piece> {
        match notation {
            'P' => Some(Piece::new(PieceType::Pawn, Side::White)),
            'R' => Some(Piece::new(PieceType::Rook, Side::White)),
            'N' => Some(Piece::new(PieceType::Knight, Side::White)),
            'B' => Some(Piece::new(PieceType::Bishop, Side::White)),
            'K' => Some(Piece::new(PieceType::King, Side::White)),
            'Q' => Some(Piece::new(PieceType::Queen, Side::White)),
            'p' => Some(Piece::new(PieceType::Pawn, Side::Black)),
            'r' => Some(Piece::new(PieceType::Rook, Side::Black)),
            'n' => Some(Piece::new(PieceType::Knight, Side::Black)),
            'b' => Some(Piece::new(PieceType::Bishop, Side::Black)),
            'k' => Some(Piece::new(PieceType::King, Side::Black)),
            'q' => Some(Piece::new(PieceType::Queen, Side::Black)),
            _ => None,
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

    #[test]
    fn from_notation() {
        assert_eq!(
            Piece::from('P').unwrap(),
            Piece::new(PieceType::Pawn, Side::White)
        );
        assert_eq!(
            Piece::from('p').unwrap(),
            Piece::new(PieceType::Pawn, Side::Black)
        );
        assert_eq!(
            Piece::from('N').unwrap(),
            Piece::new(PieceType::Knight, Side::White)
        );
        assert_eq!(
            Piece::from('n').unwrap(),
            Piece::new(PieceType::Knight, Side::Black)
        );
        assert_eq!(
            Piece::from('B').unwrap(),
            Piece::new(PieceType::Bishop, Side::White)
        );
        assert_eq!(
            Piece::from('b').unwrap(),
            Piece::new(PieceType::Bishop, Side::Black)
        );
        assert_eq!(
            Piece::from('R').unwrap(),
            Piece::new(PieceType::Rook, Side::White)
        );
        assert_eq!(
            Piece::from('r').unwrap(),
            Piece::new(PieceType::Rook, Side::Black)
        );
        assert_eq!(
            Piece::from('Q').unwrap(),
            Piece::new(PieceType::Queen, Side::White)
        );
        assert_eq!(
            Piece::from('q').unwrap(),
            Piece::new(PieceType::Queen, Side::Black)
        );
        assert_eq!(
            Piece::from('K').unwrap(),
            Piece::new(PieceType::King, Side::White)
        );
        assert_eq!(
            Piece::from('k').unwrap(),
            Piece::new(PieceType::King, Side::Black)
        );

        assert_eq!(Piece::from('a'), None);
    }
}
