use crate::chess::{
    bitboard::shift::{DIRECTION_ITEMS, KNIGHTDIR_ITEMS},
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
