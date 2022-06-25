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

    pub const fn from_squares<const LEN: usize>(sqs: [usize; LEN]) -> Self {
        let mut bb = Self(0);
        let mut i = 0;
        while i < LEN {
            assert!(sqs[i] < 64);
            bb.0 |= Bitboard::from_square(sqs[i]).0;
            i += 1;
        }
        bb
    }

    pub const fn rank(sq: usize) -> Self {
        assert!(sq < 64);
        Self(RANK_1.0 << (8 * (sq / 8)))
    }

    pub const fn file(sq: usize) -> Self {
        assert!(sq < 64);
        Self(FILE_A.0 << (sq % 8))
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
pub const FILE_B: Bitboard = Bitboard(FILE_A.0 << 1);
pub const FILE_G: Bitboard = Bitboard(FILE_A.0 << 6);
pub const FILE_H: Bitboard = Bitboard(FILE_A.0 << 7);

pub const RANK_1: Bitboard = Bitboard(0b11111111);
pub const RANK_3: Bitboard = Bitboard(RANK_1.0 << 16);
pub const RANK_6: Bitboard = Bitboard(RANK_1.0 << 40);
pub const RANK_8: Bitboard = Bitboard(RANK_1.0 << 56);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, RANK_1.0)]
    #[case(9, RANK_1.0 << 8)]
    #[case(18, RANK_1.0 << 16)]
    #[case(27, RANK_1.0 << 24)]
    #[case(36, RANK_1.0 << 32)]
    #[case(45, RANK_1.0 << 40)]
    #[case(54, RANK_1.0 << 48)]
    #[case(63, RANK_1.0 << 56)]
    pub fn rank_test(#[case] sq: usize, #[case] expected: u64) {
        assert_eq!(Bitboard(expected), Bitboard::rank(sq));
    }

    #[rstest]
    #[case(0, FILE_A.0)]
    #[case(9, FILE_A.0 << 1)]
    #[case(18, FILE_A.0 << 2)]
    #[case(27, FILE_A.0 << 3)]
    #[case(36, FILE_A.0 << 4)]
    #[case(45, FILE_A.0 << 5)]
    #[case(54, FILE_A.0 << 6)]
    #[case(63, FILE_A.0 << 7)]
    pub fn file_test(#[case] sq: usize, #[case] expected: u64) {
        assert_eq!(Bitboard(expected), Bitboard::file(sq));
    }

    #[test]
    pub fn from_squares_test() {
        let x = [0, 8, 16, 24, 32, 40, 48, 56];
        assert_eq!(Bitboard::from_squares(x), FILE_A);
    }
}
