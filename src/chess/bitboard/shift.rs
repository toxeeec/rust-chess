use super::{Bitboard, FILE_A, FILE_H};

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
}
