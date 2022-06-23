mod bits;
pub(crate) mod shift;

use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Bitboard(pub u64);

const BITBOARD_STRING_LENGTH: usize = 16 * 8 - 1;

impl Bitboard {
    pub const fn from_square(sq: usize) -> Self {
        assert!(sq < 64);
        Self(1 << sq)
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:064b}", self.0)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b: Vec<char> = format!("{:064b}", self.0).chars().rev().collect();
        let mut formatted = String::with_capacity(BITBOARD_STRING_LENGTH);
        for rank in (0..8).rev() {
            for file in 0..8 {
                formatted.push(b[rank * 8 + file] as char);
                if file < 7 {
                    formatted.push(' ');
                } else if rank > 0 {
                    formatted.push('\n');
                }
            }
        }
        write!(f, "{}", formatted)
    }
}

pub const FILE_A: Bitboard = Bitboard(0x0101010101010101);
pub const FILE_B: Bitboard = Bitboard(0x0202020202020202);
pub const FILE_G: Bitboard = Bitboard(0x4040404040404040);
pub const FILE_H: Bitboard = Bitboard(0x8080808080808080);

pub const RANK_1: Bitboard = Bitboard(0b11111111);
pub const RANK_8: Bitboard = Bitboard(0b11111111 << 56);
