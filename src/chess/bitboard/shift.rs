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

impl Bitboard {
    pub fn shift(&mut self, dir: Direction) {
        match dir {
            Direction::North => *self <<= 8,
            Direction::South => *self >>= 8,
            Direction::East => *self = (*self & !FILE_H) << 1,
            Direction::West => *self = (*self & !FILE_A) >> 1,
            Direction::NorthEast => *self = (*self & !FILE_H) << 9,
            Direction::NorthWest => *self = (*self & !FILE_A) << 7,
            Direction::SouthWest => *self = (*self & !FILE_A) >> 9,
            Direction::SouthEast => *self = (*self & !FILE_H) >> 7,
        }
    }
    pub fn knight_shift(&mut self, dir: KnightDir) {
        match dir {
            KnightDir::NNE => *self = (*self & !FILE_H) << 17,
            KnightDir::NEE => *self = (*self & !(FILE_G | FILE_H)) << 10,
            KnightDir::SEE => *self = (*self & !(FILE_G | FILE_H)) >> 6,
            KnightDir::SSE => *self = (*self & !FILE_H) >> 15,
            KnightDir::SSW => *self = (*self & !FILE_A) >> 17,
            KnightDir::SWW => *self = (*self & !(FILE_A | FILE_B)) >> 10,
            KnightDir::NWW => *self = (*self & !(FILE_A | FILE_B)) << 6,
            KnightDir::NNW => *self = (*self & !FILE_A) << 15,
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
    fn shift_test(#[case] bb: u64, #[case] direction: Direction, #[case] expected: u64) {
        let mut bb = Bitboard(bb);
        bb.shift(direction);
        let expected = Bitboard(expected);

        assert_eq!(expected, bb);
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
    fn knight_shift_test(#[case] bb: u64, #[case] direction: KnightDir, #[case] expected: u64) {
        let mut bb = Bitboard(bb);
        bb.knight_shift(direction);
        let expected = Bitboard(expected);

        assert_eq!(expected, bb);
    }
}
