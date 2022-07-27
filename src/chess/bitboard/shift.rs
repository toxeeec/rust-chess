use super::{Bitboard, FILE_A, FILE_B, FILE_G, FILE_H};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North = 8,
    East = 1,
    South = -8,
    West = -1,
    NorthEast = 9,
    NorthWest = 7,
    SouthEast = -7,
    SouthWest = -9,
}

impl Direction {
    pub const fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::NorthWest => Direction::SouthEast,
            Direction::SouthWest => Direction::NorthEast,
            Direction::SouthEast => Direction::NorthWest,
        }
    }
}

pub const DIRECTION_ITEMS: [Direction; 8] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
    Direction::NorthEast,
    Direction::NorthWest,
    Direction::SouthEast,
    Direction::SouthWest,
];

#[derive(Clone, Copy)]
pub enum KnightDir {
    Nne,
    Nee,
    See,
    Sse,
    Ssw,
    Sww,
    Nww,
    Nnw,
}

pub const KNIGHTDIR_ITEMS: [KnightDir; 8] = [
    KnightDir::Nne,
    KnightDir::Nee,
    KnightDir::See,
    KnightDir::Sse,
    KnightDir::Ssw,
    KnightDir::Sww,
    KnightDir::Nww,
    KnightDir::Nnw,
];

impl Bitboard {
    pub const fn shifted(&self, dir: Direction) -> Self {
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
    pub const fn knightdir_shifted(&self, dir: KnightDir) -> Self {
        match dir {
            KnightDir::Nne => Bitboard((self.0 & !FILE_H.0) << 17),
            KnightDir::Nee => Bitboard((self.0 & !(FILE_G.0 | FILE_H.0)) << 10),
            KnightDir::See => Bitboard((self.0 & !(FILE_G.0 | FILE_H.0)) >> 6),
            KnightDir::Sse => Bitboard((self.0 & !FILE_H.0) >> 15),
            KnightDir::Ssw => Bitboard((self.0 & !FILE_A.0) >> 17),
            KnightDir::Sww => Bitboard((self.0 & !(FILE_A.0 | FILE_B.0)) >> 10),
            KnightDir::Nww => Bitboard((self.0 & !(FILE_A.0 | FILE_B.0)) << 6),
            KnightDir::Nnw => Bitboard((self.0 & !FILE_A.0) << 15),
        }
    }

    pub const fn shifted_forward<const IS_WHITE: bool>(&self) -> Self {
        if IS_WHITE {
            self.shifted(Direction::North)
        } else {
            self.shifted(Direction::South)
        }
    }

    pub const fn shifted_forward_left<const IS_WHITE: bool>(&self) -> Self {
        if IS_WHITE {
            self.shifted(Direction::NorthWest)
        } else {
            self.shifted(Direction::SouthEast)
        }
    }

    pub const fn shifted_forward_right<const IS_WHITE: bool>(&self) -> Self {
        if IS_WHITE {
            self.shifted(Direction::NorthEast)
        } else {
            self.shifted(Direction::SouthWest)
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
    #[case(1, KnightDir::Nne, 1 << 17)]
    #[case(1 << 7, KnightDir::Nne, 0)]
    #[case(1 << 48, KnightDir::Nne, 0)]
    #[case(1 << 56, KnightDir::Nne, 0)]
    #[case(1, KnightDir::Nee, 1 << 10)]
    #[case(1 << 54, KnightDir::Nee, 0)]
    #[case(1 << 55, KnightDir::Nee, 0)]
    #[case(1 << 56, KnightDir::Nee, 0)]
    #[case(1, KnightDir::See, 0)]
    #[case(1 << 14, KnightDir::See, 0)]
    #[case(1 << 15, KnightDir::See, 0)]
    #[case(1 << 56, KnightDir::See, 1 << 50)]
    #[case(1, KnightDir::Sse, 0)]
    #[case(1 << 8, KnightDir::Sse, 0)]
    #[case(1 << 56, KnightDir::Sse, 1 << 41)]
    #[case(1 << 63, KnightDir::Sse, 0)]
    #[case(1 << 15, KnightDir::Ssw, 0)]
    #[case(1 << 7, KnightDir::Ssw, 0)]
    #[case(1 << 56, KnightDir::Ssw, 0)]
    #[case(1 << 63, KnightDir::Ssw, 1 << 46)]
    #[case(1 << 8, KnightDir::Sww, 0)]
    #[case(1 << 9, KnightDir::Sww, 0)]
    #[case(1 << 7, KnightDir::Sww, 0)]
    #[case(1 << 63, KnightDir::Sww, 1 << 53)]
    #[case(1 << 48, KnightDir::Nww, 0)]
    #[case(1 << 49, KnightDir::Nww, 0)]
    #[case(1 << 63, KnightDir::Nww, 0)]
    #[case(1 << 7, KnightDir::Nww, 1 << 13)]
    #[case(1, KnightDir::Nnw, 0)]
    #[case(1 << 7, KnightDir::Nnw, 1 << 22)]
    #[case(1 << 55, KnightDir::Nnw, 0)]
    #[case(1 << 63, KnightDir::Nnw, 0)]
    fn knightdir_shifted_test(#[case] bb: u64, #[case] dir: KnightDir, #[case] expected: u64) {
        let bb = Bitboard(bb);
        assert_eq!(Bitboard(expected), bb.knightdir_shifted(dir));
    }

    #[test]
    fn shifted_forward_white_test() {
        let bb = Bitboard(0b11111111 << 8);
        let expected = Bitboard(0b11111111 << 16);
        assert_eq!(expected, bb.shifted_forward::<true>());
    }

    #[test]
    fn shifted_forward_black_test() {
        let bb = Bitboard(0b11111111 << 8);
        let expected = Bitboard(0b11111111);
        assert_eq!(expected, bb.shifted_forward::<false>());
    }

    #[test]
    fn shifted_forward_left_white_test() {
        let bb = Bitboard(0b11111111 << 8);
        let expected = Bitboard(0b1111111 << 16);
        assert_eq!(expected, bb.shifted_forward_left::<true>());
    }

    #[test]
    fn shifted_forward_left_black_test() {
        let bb = Bitboard(0b11111111 << 8);
        let expected = Bitboard(0b11111110);
        assert_eq!(expected, bb.shifted_forward_left::<false>());
    }

    #[test]
    fn shifted_forward_right_white_test() {
        let bb = Bitboard(0b11111111 << 8);
        let expected = Bitboard(0b11111110 << 16);
        assert_eq!(expected, bb.shifted_forward_right::<true>());
    }

    #[test]
    fn shifted_forward_right_black_test() {
        let bb = Bitboard(0b11111111 << 8);
        let expected = Bitboard(0b1111111);
        assert_eq!(expected, bb.shifted_forward_right::<false>());
    }
}
