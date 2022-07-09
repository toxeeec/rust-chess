use crate::chess::{
    bitboard::shift::{Direction, DIRECTION_ITEMS, KNIGHTDIR_ITEMS},
    square::{max, min},
    Bitboard,
};

const fn knight() -> [Bitboard; 64] {
    let mut bbs = [Bitboard(0); 64];
    let mut sq = 0;
    while sq < 64 {
        let bb = Bitboard::from_square(sq);
        let mut dir = 0;
        while dir < 8 {
            bbs[sq].0 |= bb.knightdir_shifted(KNIGHTDIR_ITEMS[dir]).0;
            dir += 1;
        }
        sq += 1;
    }
    bbs
}

//TODO: castling, check, checkmate
const fn king() -> [Bitboard; 64] {
    let mut bbs = [Bitboard(0); 64];
    let mut sq = 0;
    while sq < 64 {
        let bb = Bitboard::from_square(sq);
        let mut dir = 0;
        while dir < 8 {
            bbs[sq].0 |= bb.shifted(DIRECTION_ITEMS[dir]).0;
            dir += 1;
        }
        sq += 1;
    }
    bbs
}

pub const KNIGHT: [Bitboard; 64] = knight();
pub const KING: [Bitboard; 64] = king();

const fn check_path() -> [Bitboard; 4096] {
    let mut bbs = [Bitboard(0); 4096];
    let mut king_sq = 0;
    while king_sq < 64 {
        let mut enemy_sq = 0;
        while enemy_sq < 64 {
            let diff = max(king_sq, enemy_sq) - min(king_sq, enemy_sq);
            let dir = if diff == 0 {
                enemy_sq += 1;
                continue;
            } else if diff < 8 {
                Direction::East
            } else if diff % 8 == 0 {
                Direction::North
            } else if diff % 7 == 0 {
                Direction::NorthWest
            } else if diff % 9 == 0 {
                Direction::NorthEast
            } else {
                enemy_sq += 1;
                continue;
            };
            let mut sq = king_sq;
            let mut bb = Bitboard(0);
            while sq != enemy_sq {
                if king_sq < enemy_sq {
                    sq += dir as usize;
                } else {
                    sq -= dir as usize;
                }
                bb.0 |= Bitboard::from_square(sq).0;
            }
            bbs[king_sq * 64 + enemy_sq] = bb;
            enemy_sq += 1;
        }
        king_sq += 1;
    }
    bbs
}

const CHECK_PATH: [Bitboard; 4096] = check_path();

const fn square_behind() -> [Bitboard; 4096] {
    let mut bbs = [Bitboard(0); 4096];
    let mut king_sq = 0;
    while king_sq < 64 {
        let mut enemy_sq = 0;
        while enemy_sq < 64 {
            let diff = max(king_sq, enemy_sq) - min(king_sq, enemy_sq);
            let mut dir = if diff == 0 {
                enemy_sq += 1;
                continue;
            } else if diff < 8 {
                Direction::East
            } else if diff % 8 == 0 {
                Direction::North
            } else if diff % 7 == 0 {
                Direction::NorthWest
            } else if diff % 9 == 0 {
                Direction::NorthEast
            } else {
                enemy_sq += 1;
                continue;
            };
            if king_sq < enemy_sq {
                dir = dir.opposite();
            }
            let bb = Bitboard::from_square(king_sq).shifted(dir);
            bbs[king_sq * 64 + enemy_sq] = bb;
            enemy_sq += 1;
        }
        king_sq += 1;
    }
    bbs
}

const fn pin_path() -> [Bitboard; 4096] {
    let mut bbs = CHECK_PATH;
    let behind = square_behind();
    let mut i = 0;
    while i < 4096 {
        bbs[i].0 |= behind[i].0;
        i += 1;
    }
    bbs
}

const PIN_PATH: [Bitboard; 4096] = pin_path();

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    const SQUARE_BEHIND: [Bitboard; 4096] = square_behind();

    #[test]
    fn knight_test_corner() {
        let expected = Bitboard::from_squares([10, 17]);
        assert_eq!(expected, KNIGHT[0]);
    }

    #[test]
    fn knight_test_center() {
        let expected = Bitboard::from_squares([44, 37, 21, 12, 10, 17, 33, 42]);
        assert_eq!(expected, KNIGHT[27]);
    }

    #[test]
    fn king_test_corner() {
        let expected = Bitboard::from_squares([8, 9, 1]);
        assert_eq!(expected, KING[0]);
    }

    #[test]
    fn king_test_center() {
        let expected = Bitboard::from_squares([35, 36, 28, 20, 19, 18, 26, 34]);
        assert_eq!(expected, KING[27]);
    }

    #[rstest]
    #[case(0, 0, Bitboard(0))]
    #[case(0, 3, Bitboard::from_squares([1, 2, 3]))]
    #[case(0, 24, Bitboard::from_squares([8, 16, 24]))]
    #[case(0, 27, Bitboard::from_squares([9, 18, 27]))]
    #[case(24, 3, Bitboard::from_squares([3, 10, 17]))]
    #[case(0, 25, Bitboard(0))]
    fn check_path_test(
        #[case] king_sq: usize,
        #[case] enemy_sq: usize,
        #[case] expected: Bitboard,
    ) {
        assert_eq!(expected, CHECK_PATH[king_sq * 64 + enemy_sq]);
    }

    #[rstest]
    #[case(0, 0, Bitboard(0))]
    #[case(0, 1, Bitboard(0))]
    #[case(0, 9, Bitboard(0))]
    #[case(63, 7, Bitboard(0))]
    #[case(1, 2, Bitboard::from_square(0))]
    fn square_behind_test(
        #[case] king_sq: usize,
        #[case] enemy_sq: usize,
        #[case] expected: Bitboard,
    ) {
        assert_eq!(expected, SQUARE_BEHIND[king_sq * 64 + enemy_sq]);
    }

    #[rstest]
    #[case(0, 0, Bitboard(0))]
    #[case(0, 3, Bitboard::from_squares([1, 2, 3]))]
    #[case(8, 24, Bitboard::from_squares([0, 16, 24]))]
    #[case(18, 27, Bitboard::from_squares([9, 27]))]
    #[case(24, 3, Bitboard::from_squares([3, 10, 17]))]
    #[case(0, 25, Bitboard(0))]
    fn pin_path_test(#[case] king_sq: usize, #[case] enemy_sq: usize, #[case] expected: Bitboard) {
        assert_eq!(expected, PIN_PATH[king_sq * 64 + enemy_sq]);
    }
}
