pub const A: usize = 0;
pub const B: usize = 1;
pub const C: usize = 2;
pub const D: usize = 3;
pub const E: usize = 4;
pub const F: usize = 5;
pub const G: usize = 6;
pub const H: usize = 7;

pub const LENGTH: usize = 8;

pub fn valid(file: i32) -> bool {
    file >= A as i32 && file <= H as i32
}

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
        _ => '?',
    }
}

pub fn from_char(file: char) -> Option<usize> {
    match file {
        'a' => Some(A),
        'b' => Some(B),
        'c' => Some(C),
        'd' => Some(D),
        'e' => Some(E),
        'f' => Some(F),
        'g' => Some(G),
        'h' => Some(H),
        _ => None,
    }
}
