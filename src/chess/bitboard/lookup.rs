use super::{
    shift::{DIRECTION_ITEMS, KNIGHTDIR_ITEMS},
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
        let expected = Bitboard::from_square(10) | Bitboard::from_square(17);
        assert_eq!(expected, KNIGHT[0]);
    }

    #[test]
    fn knight_test_center() {
        let expected = Bitboard::from_square(44)
            | Bitboard::from_square(37)
            | Bitboard::from_square(21)
            | Bitboard::from_square(12)
            | Bitboard::from_square(10)
            | Bitboard::from_square(17)
            | Bitboard::from_square(33)
            | Bitboard::from_square(42);
        assert_eq!(expected, KNIGHT[27]);
    }

    #[test]
    fn king_test_corner() {
        let expected =
            Bitboard::from_square(8) | Bitboard::from_square(9) | Bitboard::from_square(1);
        assert_eq!(expected, KING[0]);
    }

    #[test]
    fn king_test_center() {
        let expected = Bitboard::from_square(35)
            | Bitboard::from_square(36)
            | Bitboard::from_square(28)
            | Bitboard::from_square(20)
            | Bitboard::from_square(19)
            | Bitboard::from_square(18)
            | Bitboard::from_square(26)
            | Bitboard::from_square(34);
        assert_eq!(expected, KING[27]);
    }
}
