use super::{Bitboard, FILE_A, FILE_B, FILE_G, FILE_H};

pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Clone, Copy)]
pub enum KnightDir {
    NNE,
    NEE,
    SEE,
    SSE,
    SSW,
    SWW,
    NWW,
    NNW,
}

pub const KNIGHTDIR_ITEMS: [KnightDir; 8] = [
    KnightDir::NNE,
    KnightDir::NEE,
    KnightDir::SEE,
    KnightDir::SSE,
    KnightDir::SSW,
    KnightDir::SWW,
    KnightDir::NWW,
    KnightDir::NNW,
];

impl Bitboard {
    pub const fn shifted(self, dir: Direction) -> Self {
        match dir {
            Direction::North => Bitboard(self.0 << 8),
            Direction::South => Bitboard(self.0 >> 8),
            Direction::East => Bitboard((self.0 & !FILE_H.0) << 1),
            Direction::West => Bitboard((self.0 & !FILE_A.0) >> 1),
            Direction::NorthEast => Bitboard((self.0 & !FILE_H.0) << 9),
            Direction::NorthWest => Bitboard((self.0 & !FILE_A.0) << 7),
            Direction::SouthWest => Bitboard((self.0 & !FILE_A.0) >> 9),
            Direction::SouthEast => Bitboard((self.0 & !FILE_H.0) >> 7),
        }
    }
    pub const fn knightdir_shifted(self, dir: KnightDir) -> Self {
        match dir {
            KnightDir::NNE => Bitboard((self.0 & !FILE_H.0) << 17),
            KnightDir::NEE => Bitboard((self.0 & !(FILE_G.0 | FILE_H.0)) << 10),
            KnightDir::SEE => Bitboard((self.0 & !(FILE_G.0 | FILE_H.0)) >> 6),
            KnightDir::SSE => Bitboard((self.0 & !FILE_H.0) >> 15),
            KnightDir::SSW => Bitboard((self.0 & !FILE_A.0) >> 17),
            KnightDir::SWW => Bitboard((self.0 & !(FILE_A.0 | FILE_B.0)) >> 10),
            KnightDir::NWW => Bitboard((self.0 & !(FILE_A.0 | FILE_B.0)) << 6),
            KnightDir::NNW => Bitboard((self.0 & !FILE_A.0) << 15),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0b11111111 << 8, Direction::North, 0b11111111 << 16)]
    #[case(0b11111111 << 56, Direction::North, 0)]
    #[case(0b11111111 << 8, Direction::South, 0b11111111)]
    #[case(0b11111111, Direction::South, 0)]
    #[case(0b11111111 << 8, Direction::East, 0b11111110 << 8)]
    #[case(0b11111111 << 8, Direction::West, 0b1111111 << 8)]
    #[case(0b11111111 << 8, Direction::NorthEast, 0b11111110 << 16)]
    #[case(0b11111111 << 8, Direction::NorthWest, 0b1111111 << 16)]
    #[case(0b11111111 << 8, Direction::SouthEast, 0b11111110)]
    #[case(0b11111111 << 8, Direction::SouthWest, 0b1111111)]
    fn shifted_test(#[case] bb: u64, #[case] dir: Direction, #[case] expected: u64) {
        let bb = Bitboard(bb);
        assert_eq!(Bitboard(expected), bb.shifted(dir));
    }

    #[rstest]
    #[case(1, KnightDir::NNE, 1 << 17)]
    #[case(1 << 7, KnightDir::NNE, 0)]
    #[case(1 << 48, KnightDir::NNE, 0)]
    #[case(1 << 56, KnightDir::NNE, 0)]
    #[case(1, KnightDir::NEE, 1 << 10)]
    #[case(1 << 54, KnightDir::NEE, 0)]
    #[case(1 << 55, KnightDir::NEE, 0)]
    #[case(1 << 56, KnightDir::NEE, 0)]
    #[case(1, KnightDir::SEE, 0)]
    #[case(1 << 14, KnightDir::SEE, 0)]
    #[case(1 << 15, KnightDir::SEE, 0)]
    #[case(1 << 56, KnightDir::SEE, 1 << 50)]
    #[case(1, KnightDir::SSE, 0)]
    #[case(1 << 8, KnightDir::SSE, 0)]
    #[case(1 << 56, KnightDir::SSE, 1 << 41)]
    #[case(1 << 63, KnightDir::SSE, 0)]
    #[case(1 << 15, KnightDir::SSW, 0)]
    #[case(1 << 7, KnightDir::SSW, 0)]
    #[case(1 << 56, KnightDir::SSW, 0)]
    #[case(1 << 63, KnightDir::SSW, 1 << 46)]
    #[case(1 << 8, KnightDir::SWW, 0)]
    #[case(1 << 9, KnightDir::SWW, 0)]
    #[case(1 << 7, KnightDir::SWW, 0)]
    #[case(1 << 63, KnightDir::SWW, 1 << 53)]
    #[case(1 << 48, KnightDir::NWW, 0)]
    #[case(1 << 49, KnightDir::NWW, 0)]
    #[case(1 << 63, KnightDir::NWW, 0)]
    #[case(1 << 7, KnightDir::NWW, 1 << 13)]
    #[case(1, KnightDir::NNW, 0)]
    #[case(1 << 7, KnightDir::NNW, 1 << 22)]
    #[case(1 << 55, KnightDir::NNW, 0)]
    #[case(1 << 63, KnightDir::NNW, 0)]
    fn knight_shift_test(#[case] bb: u64, #[case] dir: KnightDir, #[case] expected: u64) {
        let bb = Bitboard(bb);

        assert_eq!(Bitboard(expected), bb.knightdir_shifted(dir));
    }
}
