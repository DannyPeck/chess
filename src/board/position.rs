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
pub struct Position {
  position: usize
}

impl Position {
  pub fn from(position: usize) -> Option<Position> {
    if position <= H8 {
      Some(Position { position } )
    } else {
      None
    }
  }

  pub fn from_file_and_rank(file: usize, rank: usize) -> Option<Position> {
    if file <= file::H && rank <= rank::EIGHT {
      let position = (rank * 8) + file;
      let position = position as usize;
      Some(Position { position })
    } else {
      None
    }
  }

  pub fn from_move(start: &Position, piece_move: Move) -> Option<Position> {
    let new_file = start.file() as i32 + piece_move.file_offset;
    let new_rank = start.rank() as i32 + piece_move.rank_offset;

    if new_file < 0 || new_rank < 0 {
      return None;
    }

    Position::from_file_and_rank(new_file as usize, new_rank as usize)
  }

  pub fn value(&self) -> usize {
    self.position
  }

  pub fn rank(&self) -> usize {
    self.position / 8
  }

  pub fn file(&self) -> usize {
    self.position % 8
  }
}

pub fn file_to_char(file: usize) -> char {
  match file {
    file::A => 'a',
    file::B => 'b',
    file::C => 'c',
    file::D => 'd',
    file::E => 'e',
    file::F => 'f',
    file::G => 'g',
    file::H => 'h',
    _ => '?'
  }
}

pub fn rank_to_char(rank: usize) -> char {
  match rank {
    rank::ONE => '1',
    rank::TWO => '2',
    rank::THREE => '3',
    rank::FOUR => '4',
    rank::FIVE => '5',
    rank::SIX => '6',
    rank::SEVEN => '7',
    rank::EIGHT => '8',
    _ => '?'
  }
}

impl std::fmt::Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", file_to_char(self.file()), rank_to_char(self.rank()))
  }
}

pub struct Move {
  pub file_offset: i32,
  pub rank_offset: i32
}

impl Move {
  pub fn new(file_offset: i32, rank_offset: i32) -> Move {
    Move {
      file_offset,
      rank_offset
    }
  }
}

impl Position {
  pub fn a1() -> Position { Position { position: A1 } }
  pub fn a2() -> Position { Position { position: A2 } }
  pub fn a3() -> Position { Position { position: A3 } }
  pub fn a4() -> Position { Position { position: A4 } }
  pub fn a5() -> Position { Position { position: A5 } }
  pub fn a6() -> Position { Position { position: A6 } }
  pub fn a7() -> Position { Position { position: A7 } }
  pub fn a8() -> Position { Position { position: A8 } }
  pub fn b1() -> Position { Position { position: B1 } }
  pub fn b2() -> Position { Position { position: B2 } }
  pub fn b3() -> Position { Position { position: B3 } }
  pub fn b4() -> Position { Position { position: B4 } }
  pub fn b5() -> Position { Position { position: B5 } }
  pub fn b6() -> Position { Position { position: B6 } }
  pub fn b7() -> Position { Position { position: B7 } }
  pub fn b8() -> Position { Position { position: B8 } }
  pub fn c1() -> Position { Position { position: C1 } }
  pub fn c2() -> Position { Position { position: C2 } }
  pub fn c3() -> Position { Position { position: C3 } }
  pub fn c4() -> Position { Position { position: C4 } }
  pub fn c5() -> Position { Position { position: C5 } }
  pub fn c6() -> Position { Position { position: C6 } }
  pub fn c7() -> Position { Position { position: C7 } }
  pub fn c8() -> Position { Position { position: C8 } }
  pub fn d1() -> Position { Position { position: D1 } }
  pub fn d2() -> Position { Position { position: D2 } }
  pub fn d3() -> Position { Position { position: D3 } }
  pub fn d4() -> Position { Position { position: D4 } }
  pub fn d5() -> Position { Position { position: D5 } }
  pub fn d6() -> Position { Position { position: D6 } }
  pub fn d7() -> Position { Position { position: D7 } }
  pub fn d8() -> Position { Position { position: D8 } }
  pub fn e1() -> Position { Position { position: E1 } }
  pub fn e2() -> Position { Position { position: E2 } }
  pub fn e3() -> Position { Position { position: E3 } }
  pub fn e4() -> Position { Position { position: E4 } }
  pub fn e5() -> Position { Position { position: E5 } }
  pub fn e6() -> Position { Position { position: E6 } }
  pub fn e7() -> Position { Position { position: E7 } }
  pub fn e8() -> Position { Position { position: E8 } }
  pub fn f1() -> Position { Position { position: F1 } }
  pub fn f2() -> Position { Position { position: F2 } }
  pub fn f3() -> Position { Position { position: F3 } }
  pub fn f4() -> Position { Position { position: F4 } }
  pub fn f5() -> Position { Position { position: F5 } }
  pub fn f6() -> Position { Position { position: F6 } }
  pub fn f7() -> Position { Position { position: F7 } }
  pub fn f8() -> Position { Position { position: F8 } }
  pub fn g1() -> Position { Position { position: G1 } }
  pub fn g2() -> Position { Position { position: G2 } }
  pub fn g3() -> Position { Position { position: G3 } }
  pub fn g4() -> Position { Position { position: G4 } }
  pub fn g5() -> Position { Position { position: G5 } }
  pub fn g6() -> Position { Position { position: G6 } }
  pub fn g7() -> Position { Position { position: G7 } }
  pub fn g8() -> Position { Position { position: G8 } }
  pub fn h1() -> Position { Position { position: H1 } }
  pub fn h2() -> Position { Position { position: H2 } }
  pub fn h3() -> Position { Position { position: H3 } }
  pub fn h4() -> Position { Position { position: H4 } }
  pub fn h5() -> Position { Position { position: H5 } }
  pub fn h6() -> Position { Position { position: H6 } }
  pub fn h7() -> Position { Position { position: H7 } }
  pub fn h8() -> Position { Position { position: H8 } }
}