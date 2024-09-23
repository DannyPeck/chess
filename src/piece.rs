#[macro_export]
macro_rules! piece {
    ( $piece_type:ident, $side:ident ) => {
        Piece::new(PieceType::$piece_type, Side::$side)
    };
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn value(&self) -> i32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 0,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PromotionType {
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl PromotionType {
    pub fn to_piece_type(&self) -> PieceType {
        match self {
            PromotionType::Knight => PieceType::Knight,
            PromotionType::Bishop => PieceType::Bishop,
            PromotionType::Rook => PieceType::Rook,
            PromotionType::Queen => PieceType::Queen,
        }
    }

    pub fn from_coordinate(notation: char) -> Option<PromotionType> {
        match notation {
            'q' => Some(PromotionType::Queen),
            'n' => Some(PromotionType::Knight),
            'b' => Some(PromotionType::Bishop),
            'r' => Some(PromotionType::Rook),
            _ => None,
        }
    }

    pub fn to_algebraic(&self) -> char {
        match self {
            PromotionType::Knight => 'N',
            PromotionType::Bishop => 'B',
            PromotionType::Rook => 'R',
            PromotionType::Queen => 'Q',
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
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

    pub fn opponent(&self) -> Self {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
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

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
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
            'P' => Some(piece!(Pawn, White)),
            'R' => Some(piece!(Rook, White)),
            'N' => Some(piece!(Knight, White)),
            'B' => Some(piece!(Bishop, White)),
            'K' => Some(piece!(King, White)),
            'Q' => Some(piece!(Queen, White)),
            'p' => Some(piece!(Pawn, Black)),
            'r' => Some(piece!(Rook, Black)),
            'n' => Some(piece!(Knight, Black)),
            'b' => Some(piece!(Bishop, Black)),
            'k' => Some(piece!(King, Black)),
            'q' => Some(piece!(Queen, Black)),
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
        assert_eq!(piece!(Pawn, White).to_string(), "P");
        assert_eq!(piece!(Pawn, Black).to_string(), "p");
        assert_eq!(piece!(Knight, White).to_string(), "N");
        assert_eq!(piece!(Knight, Black).to_string(), "n");
        assert_eq!(piece!(Bishop, White).to_string(), "B");
        assert_eq!(piece!(Bishop, Black).to_string(), "b");
        assert_eq!(piece!(Rook, White).to_string(), "R");
        assert_eq!(piece!(Rook, Black).to_string(), "r");
        assert_eq!(piece!(Queen, White).to_string(), "Q");
        assert_eq!(piece!(Queen, Black).to_string(), "q");
        assert_eq!(piece!(King, White).to_string(), "K");
        assert_eq!(piece!(King, Black).to_string(), "k");
    }

    #[test]
    fn from_notation() {
        assert_eq!(Piece::from('P').unwrap(), piece!(Pawn, White));
        assert_eq!(Piece::from('p').unwrap(), piece!(Pawn, Black));
        assert_eq!(Piece::from('N').unwrap(), piece!(Knight, White));
        assert_eq!(Piece::from('n').unwrap(), piece!(Knight, Black));
        assert_eq!(Piece::from('B').unwrap(), piece!(Bishop, White));
        assert_eq!(Piece::from('b').unwrap(), piece!(Bishop, Black));
        assert_eq!(Piece::from('R').unwrap(), piece!(Rook, White));
        assert_eq!(Piece::from('r').unwrap(), piece!(Rook, Black));
        assert_eq!(Piece::from('Q').unwrap(), piece!(Queen, White));
        assert_eq!(Piece::from('q').unwrap(), piece!(Queen, Black));
        assert_eq!(Piece::from('K').unwrap(), piece!(King, White));
        assert_eq!(Piece::from('k').unwrap(), piece!(King, Black));

        assert_eq!(Piece::from('a'), None);
    }
}
