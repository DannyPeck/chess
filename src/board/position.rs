use super::file;
use super::rank;

pub const A1: usize = 0;
pub const B1: usize = 1;
pub const C1: usize = 2;
pub const D1: usize = 3;
pub const E1: usize = 4;
pub const F1: usize = 5;
pub const G1: usize = 6;
pub const H1: usize = 7;
pub const A2: usize = 8;
pub const B2: usize = 9;
pub const C2: usize = 10;
pub const D2: usize = 11;
pub const E2: usize = 12;
pub const F2: usize = 13;
pub const G2: usize = 14;
pub const H2: usize = 15;
pub const A3: usize = 16;
pub const B3: usize = 17;
pub const C3: usize = 18;
pub const D3: usize = 19;
pub const E3: usize = 20;
pub const F3: usize = 21;
pub const G3: usize = 22;
pub const H3: usize = 23;
pub const A4: usize = 24;
pub const B4: usize = 25;
pub const C4: usize = 26;
pub const D4: usize = 27;
pub const E4: usize = 28;
pub const F4: usize = 29;
pub const G4: usize = 30;
pub const H4: usize = 31;
pub const A5: usize = 32;
pub const B5: usize = 33;
pub const C5: usize = 34;
pub const D5: usize = 35;
pub const E5: usize = 36;
pub const F5: usize = 37;
pub const G5: usize = 38;
pub const H5: usize = 39;
pub const A6: usize = 40;
pub const B6: usize = 41;
pub const C6: usize = 42;
pub const D6: usize = 43;
pub const E6: usize = 44;
pub const F6: usize = 45;
pub const G6: usize = 46;
pub const H6: usize = 47;
pub const A7: usize = 48;
pub const B7: usize = 49;
pub const C7: usize = 50;
pub const D7: usize = 51;
pub const E7: usize = 52;
pub const F7: usize = 53;
pub const G7: usize = 54;
pub const H7: usize = 55;
pub const A8: usize = 56;
pub const B8: usize = 57;
pub const C8: usize = 58;
pub const D8: usize = 59;
pub const E8: usize = 60;
pub const F8: usize = 61;
pub const G8: usize = 62;
pub const H8: usize = 63;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Position(usize);

impl Position {
    pub fn from_file_and_rank(file: usize, rank: usize) -> Position {
        if !file::valid(file as i32) || !rank::valid(rank as i32) {
            panic!("Passed an invalid file or rank value into from_file_and_rank().");
        }

        let position = (rank * 8) + file;
        Position(position)
    }

    pub fn from_offset(start: &Position, offset: &Offset) -> Option<Position> {
        let new_file = start.file() as i32 + offset.file_offset;
        let new_rank = start.rank() as i32 + offset.rank_offset;

        if !file::valid(new_file) || !rank::valid(new_rank) {
            return None;
        }

        Some(Position::from_file_and_rank(
            new_file as usize,
            new_rank as usize,
        ))
    }

    pub fn from_str(position: &str) -> Option<Position> {
        if position.len() != 2 {
            return None;
        }

        let position = position.to_lowercase();

        let file: char = position.chars().nth(0).unwrap();
        let rank: char = position.chars().nth(1).unwrap();

        match (file::from_char(file), rank::from_char(rank)) {
            (Some(file), Some(rank)) => Some(Position::from_file_and_rank(file, rank)),
            _ => None,
        }
    }

    pub fn value(&self) -> usize {
        self.0
    }

    pub fn rank(&self) -> usize {
        self.0 / rank::LENGTH
    }

    pub fn file(&self) -> usize {
        self.0 % file::LENGTH
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            file::to_char(self.file()),
            rank::to_char(self.rank())
        )
    }
}

#[derive(Debug)]
pub struct Offset {
    pub file_offset: i32,
    pub rank_offset: i32,
}

impl Offset {
    pub fn new(file_offset: i32, rank_offset: i32) -> Offset {
        Offset {
            file_offset,
            rank_offset,
        }
    }
}

impl Position {
    pub fn a1() -> Position {
        Position(A1)
    }
    pub fn a2() -> Position {
        Position(A2)
    }
    pub fn a3() -> Position {
        Position(A3)
    }
    pub fn a4() -> Position {
        Position(A4)
    }
    pub fn a5() -> Position {
        Position(A5)
    }
    pub fn a6() -> Position {
        Position(A6)
    }
    pub fn a7() -> Position {
        Position(A7)
    }
    pub fn a8() -> Position {
        Position(A8)
    }
    pub fn b1() -> Position {
        Position(B1)
    }
    pub fn b2() -> Position {
        Position(B2)
    }
    pub fn b3() -> Position {
        Position(B3)
    }
    pub fn b4() -> Position {
        Position(B4)
    }
    pub fn b5() -> Position {
        Position(B5)
    }
    pub fn b6() -> Position {
        Position(B6)
    }
    pub fn b7() -> Position {
        Position(B7)
    }
    pub fn b8() -> Position {
        Position(B8)
    }
    pub fn c1() -> Position {
        Position(C1)
    }
    pub fn c2() -> Position {
        Position(C2)
    }
    pub fn c3() -> Position {
        Position(C3)
    }
    pub fn c4() -> Position {
        Position(C4)
    }
    pub fn c5() -> Position {
        Position(C5)
    }
    pub fn c6() -> Position {
        Position(C6)
    }
    pub fn c7() -> Position {
        Position(C7)
    }
    pub fn c8() -> Position {
        Position(C8)
    }
    pub fn d1() -> Position {
        Position(D1)
    }
    pub fn d2() -> Position {
        Position(D2)
    }
    pub fn d3() -> Position {
        Position(D3)
    }
    pub fn d4() -> Position {
        Position(D4)
    }
    pub fn d5() -> Position {
        Position(D5)
    }
    pub fn d6() -> Position {
        Position(D6)
    }
    pub fn d7() -> Position {
        Position(D7)
    }
    pub fn d8() -> Position {
        Position(D8)
    }
    pub fn e1() -> Position {
        Position(E1)
    }
    pub fn e2() -> Position {
        Position(E2)
    }
    pub fn e3() -> Position {
        Position(E3)
    }
    pub fn e4() -> Position {
        Position(E4)
    }
    pub fn e5() -> Position {
        Position(E5)
    }
    pub fn e6() -> Position {
        Position(E6)
    }
    pub fn e7() -> Position {
        Position(E7)
    }
    pub fn e8() -> Position {
        Position(E8)
    }
    pub fn f1() -> Position {
        Position(F1)
    }
    pub fn f2() -> Position {
        Position(F2)
    }
    pub fn f3() -> Position {
        Position(F3)
    }
    pub fn f4() -> Position {
        Position(F4)
    }
    pub fn f5() -> Position {
        Position(F5)
    }
    pub fn f6() -> Position {
        Position(F6)
    }
    pub fn f7() -> Position {
        Position(F7)
    }
    pub fn f8() -> Position {
        Position(F8)
    }
    pub fn g1() -> Position {
        Position(G1)
    }
    pub fn g2() -> Position {
        Position(G2)
    }
    pub fn g3() -> Position {
        Position(G3)
    }
    pub fn g4() -> Position {
        Position(G4)
    }
    pub fn g5() -> Position {
        Position(G5)
    }
    pub fn g6() -> Position {
        Position(G6)
    }
    pub fn g7() -> Position {
        Position(G7)
    }
    pub fn g8() -> Position {
        Position(G8)
    }
    pub fn h1() -> Position {
        Position(H1)
    }
    pub fn h2() -> Position {
        Position(H2)
    }
    pub fn h3() -> Position {
        Position(H3)
    }
    pub fn h4() -> Position {
        Position(H4)
    }
    pub fn h5() -> Position {
        Position(H5)
    }
    pub fn h6() -> Position {
        Position(H6)
    }
    pub fn h7() -> Position {
        Position(H7)
    }
    pub fn h8() -> Position {
        Position(H8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_file_and_rank_valid() -> Result<(), String> {
        Position::from_file_and_rank(0, 0);
        Position::from_file_and_rank(0, 7);
        Position::from_file_and_rank(7, 0);
        Position::from_file_and_rank(7, 7);

        {
            let position = Position::from_file_and_rank(file::A, rank::ONE);
            assert_eq!(position.value(), 0);
        }

        {
            let position = Position::from_file_and_rank(file::H, rank::EIGHT);
            assert_eq!(position.value(), 63);
        }

        {
            let position = Position::from_file_and_rank(file::C, rank::SIX);
            assert_eq!(position.value(), 42);
        }

        Ok(())
    }

    #[test]
    #[should_panic]
    fn from_file_and_rank_invalid_file() {
        Position::from_file_and_rank(8, 0);
    }

    #[test]
    #[should_panic]
    fn from_file_and_rank_invalid_rank() {
        Position::from_file_and_rank(0, 8);
    }

    #[test]
    fn from_offset() {
        // Valid forward file move
        {
            let new_position = Position::from_offset(&Position::a4(), &Offset::new(1, 0));
            assert!(new_position.is_some());
            assert_eq!(new_position.unwrap(), Position::b4());
        }

        // Valid backward file move
        {
            let new_position = Position::from_offset(&Position::e4(), &Offset::new(-2, 0));
            assert!(new_position.is_some());
            assert_eq!(new_position.unwrap(), Position::c4());
        }

        // Valid forward rank move
        {
            let new_position = Position::from_offset(&Position::h3(), &Offset::new(0, 5));
            assert!(new_position.is_some());
            assert_eq!(new_position.unwrap(), Position::h8());
        }

        // Valid backwards rank move
        {
            let new_position = Position::from_offset(&Position::d6(), &Offset::new(0, -1));
            assert!(new_position.is_some());
            assert_eq!(new_position.unwrap(), Position::d5());
        }

        // Valid no-op move
        {
            let new_position = Position::from_offset(&Position::d6(), &Offset::new(0, 0));
            assert!(new_position.is_some());
            assert_eq!(new_position.unwrap(), Position::d6());
        }

        // Invalid forward file move
        {
            let new_position = Position::from_offset(&Position::h4(), &Offset::new(1, 0));
            assert!(new_position.is_none());
        }

        // Invalid backward file move
        {
            let new_position = Position::from_offset(&Position::a4(), &Offset::new(-1, 0));
            assert!(new_position.is_none());
        }

        // Invalid forward rank move
        {
            let new_position = Position::from_offset(&Position::d8(), &Offset::new(0, 1));
            assert!(new_position.is_none());
        }

        // Invalid backward rank move
        {
            let new_position = Position::from_offset(&Position::d2(), &Offset::new(0, -3));
            assert!(new_position.is_none());
        }
    }

    #[test]
    fn from_str() {
        // Valid positions
        assert_eq!(Position::from_str("a1").unwrap(), Position::a1());
        assert_eq!(Position::from_str("b2").unwrap(), Position::b2());
        assert_eq!(Position::from_str("c3").unwrap(), Position::c3());
        assert_eq!(Position::from_str("d4").unwrap(), Position::d4());
        assert_eq!(Position::from_str("e5").unwrap(), Position::e5());
        assert_eq!(Position::from_str("f6").unwrap(), Position::f6());
        assert_eq!(Position::from_str("g7").unwrap(), Position::g7());
        assert_eq!(Position::from_str("h8").unwrap(), Position::h8());

        // Valid case insensitive
        assert_eq!(Position::from_str("A8").unwrap(), Position::a8());
        assert_eq!(Position::from_str("B7").unwrap(), Position::b7());
        assert_eq!(Position::from_str("C6").unwrap(), Position::c6());
        assert_eq!(Position::from_str("D5").unwrap(), Position::d5());
        assert_eq!(Position::from_str("E4").unwrap(), Position::e4());
        assert_eq!(Position::from_str("F3").unwrap(), Position::f3());
        assert_eq!(Position::from_str("G2").unwrap(), Position::g2());
        assert_eq!(Position::from_str("H1").unwrap(), Position::h1());

        // Invalid positions
        assert_eq!(Position::from_str("a10"), None);
        assert_eq!(Position::from_str("b9"), None);
        assert_eq!(Position::from_str("b"), None);
        assert_eq!(Position::from_str("3"), None);
        assert_eq!(Position::from_str("i"), None);
        assert_eq!(Position::from_str("z"), None);
        assert_eq!(Position::from_str("3b"), None);
        assert_eq!(Position::from_str("h0"), None);
        assert_eq!(Position::from_str(""), None);
    }

    #[test]
    fn value() {
        assert_eq!(Position::a1().value(), 0);
        assert_eq!(Position::b2().value(), 9);
        assert_eq!(Position::c3().value(), 18);
        assert_eq!(Position::d4().value(), 27);
        assert_eq!(Position::e5().value(), 36);
        assert_eq!(Position::f6().value(), 45);
        assert_eq!(Position::g7().value(), 54);
        assert_eq!(Position::h8().value(), 63);
    }

    #[test]
    fn rank() {
        assert_eq!(Position::a1().rank(), rank::ONE);
        assert_eq!(Position::b2().rank(), rank::TWO);
        assert_eq!(Position::c3().rank(), rank::THREE);
        assert_eq!(Position::d4().rank(), rank::FOUR);
        assert_eq!(Position::e5().rank(), rank::FIVE);
        assert_eq!(Position::f6().rank(), rank::SIX);
        assert_eq!(Position::g7().rank(), rank::SEVEN);
        assert_eq!(Position::h8().rank(), rank::EIGHT);
    }

    #[test]
    fn file() {
        assert_eq!(Position::a1().file(), file::A);
        assert_eq!(Position::b2().file(), file::B);
        assert_eq!(Position::c3().file(), file::C);
        assert_eq!(Position::d4().file(), file::D);
        assert_eq!(Position::e5().file(), file::E);
        assert_eq!(Position::f6().file(), file::F);
        assert_eq!(Position::g7().file(), file::G);
        assert_eq!(Position::h8().file(), file::H);
    }

    #[test]
    fn to_string() {
        assert_eq!(Position::a1().to_string(), "a1");
        assert_eq!(Position::b2().to_string(), "b2");
        assert_eq!(Position::c3().to_string(), "c3");
        assert_eq!(Position::d4().to_string(), "d4");
        assert_eq!(Position::e5().to_string(), "e5");
        assert_eq!(Position::f6().to_string(), "f6");
        assert_eq!(Position::g7().to_string(), "g7");
        assert_eq!(Position::h8().to_string(), "h8");
    }
}
