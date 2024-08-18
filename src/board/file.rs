pub const A: usize = 0;
pub const B: usize = 1;
pub const C: usize = 2;
pub const D: usize = 3;
pub const E: usize = 4;
pub const F: usize = 5;
pub const G: usize = 6;
pub const H: usize = 7;

pub const LENGTH: usize = 8;

pub fn to_char(file: usize) -> char {
  match file {
    A => 'a',
    B => 'b',
    C => 'c',
    D => 'd',
    E => 'e',
    F => 'f',
    G => 'g',
    H => 'h',
    _ => '?'
  }
}