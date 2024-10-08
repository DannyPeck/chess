pub const ONE: usize = 0;
pub const TWO: usize = 1;
pub const THREE: usize = 2;
pub const FOUR: usize = 3;
pub const FIVE: usize = 4;
pub const SIX: usize = 5;
pub const SEVEN: usize = 6;
pub const EIGHT: usize = 7;

pub const LENGTH: usize = 8;

pub fn valid(rank: i32) -> bool {
    rank >= ONE as i32 && rank <= EIGHT as i32
}

pub fn to_char(rank: usize) -> char {
    match rank {
        ONE => '1',
        TWO => '2',
        THREE => '3',
        FOUR => '4',
        FIVE => '5',
        SIX => '6',
        SEVEN => '7',
        EIGHT => '8',
        _ => '?',
    }
}

pub fn from_char(rank: char) -> Option<usize> {
    match rank {
        '1' => Some(ONE),
        '2' => Some(TWO),
        '3' => Some(THREE),
        '4' => Some(FOUR),
        '5' => Some(FIVE),
        '6' => Some(SIX),
        '7' => Some(SEVEN),
        '8' => Some(EIGHT),
        _ => None,
    }
}
