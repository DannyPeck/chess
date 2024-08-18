use crate::board::{file, rank, position::*};

#[test]
fn from() {
  assert!(Position::from(0).is_some());
  assert!(Position::from(1).is_some());
  assert!(Position::from(63).is_some());
  assert!(Position::from(64).is_none());
}

#[test]
fn from_file_and_rank() {
  assert!(Position::from_file_and_rank(0, 0).is_some());
  assert!(Position::from_file_and_rank(0, 7).is_some());
  assert!(Position::from_file_and_rank(7, 0).is_some());
  assert!(Position::from_file_and_rank(7, 7).is_some());

  assert!(Position::from_file_and_rank(0, 8).is_none());
  assert!(Position::from_file_and_rank(8, 0).is_none());

  {
    let position = Position::from_file_and_rank(file::A, rank::ONE);
    assert!(position.is_some());
    assert_eq!(position.unwrap().value(), 0);
  }

  {
    let position = Position::from_file_and_rank(file::H, rank::EIGHT);
    assert!(position.is_some());
    assert_eq!(position.unwrap().value(), 63);
  }

  {
    let position = Position::from_file_and_rank(file::C, rank::SIX);
    assert!(position.is_some());
    assert_eq!(position.unwrap().value(), 42);
  }
}

#[test]
fn from_valid_file_and_rank_valid() -> Result<(), String> {
  Position::from_valid_file_and_rank(0, 0);
  Position::from_valid_file_and_rank(0, 7);
  Position::from_valid_file_and_rank(7, 0);
  Position::from_valid_file_and_rank(7, 7);

  {
    let position = Position::from_valid_file_and_rank(file::A, rank::ONE);
    assert_eq!(position.value(), 0);
  }

  {
    let position = Position::from_valid_file_and_rank(file::H, rank::EIGHT);
    assert_eq!(position.value(), 63);
  }

  {
    let position = Position::from_valid_file_and_rank(file::C, rank::SIX);
    assert_eq!(position.value(), 42);
  }

  Ok(())
}

#[test]
#[should_panic]
fn from_valid_file_and_rank_invalid_file() {
  Position::from_valid_file_and_rank(8, 0);
}

#[test]
#[should_panic]
fn from_valid_file_and_rank_invalid_rank() {
  Position::from_valid_file_and_rank(0, 8);
}

#[test]
fn from_offset() {
  // Valid forward file move
  {
    let new_position = Position::from_offset(&Position::a4(), PositionOffset::new(1, 0));
    assert!(new_position.is_some());
    assert_eq!(new_position.unwrap(), Position::b4());
  }

  // Valid backward file move
  {
    let new_position = Position::from_offset(&Position::e4(), PositionOffset::new(-2, 0));
    assert!(new_position.is_some());
    assert_eq!(new_position.unwrap(), Position::c4());
  }

  // Valid forward rank move
  {
    let new_position = Position::from_offset(&Position::h3(), PositionOffset::new(0, 5));
    assert!(new_position.is_some());
    assert_eq!(new_position.unwrap(), Position::h8());
  }

  // Valid backwards rank move
  {
    let new_position = Position::from_offset(&Position::d6(), PositionOffset::new(0, -1));
    assert!(new_position.is_some());
    assert_eq!(new_position.unwrap(), Position::d5());
  }

  // Valid no-op move
  {
    let new_position = Position::from_offset(&Position::d6(), PositionOffset::new(0, 0));
    assert!(new_position.is_some());
    assert_eq!(new_position.unwrap(), Position::d6());
  }

  // Invalid forward file move
  {
    let new_position = Position::from_offset(&Position::h4(), PositionOffset::new(1, 0));
    assert!(new_position.is_none());
  }

  // Invalid backward file move
  {
    let new_position = Position::from_offset(&Position::a4(), PositionOffset::new(-1, 0));
    assert!(new_position.is_none());
  }

  // Invalid forward rank move
  {
    let new_position = Position::from_offset(&Position::d8(), PositionOffset::new(0, 1));
    assert!(new_position.is_none());
  }

  // Invalid backward rank move
  {
    let new_position = Position::from_offset(&Position::d2(), PositionOffset::new(0, -3));
    assert!(new_position.is_none());
  }
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