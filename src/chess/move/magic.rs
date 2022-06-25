use crate::chess::{
    bitboard::{shift::Direction, FILE_A, FILE_H, RANK_1, RANK_8},
    square::{is_valid, SQUARES_DISTANCES},
    Bitboard,
};

const ROOK_DIRS: [isize; 4] = [
    Direction::North as isize,
    Direction::East as isize,
    Direction::South as isize,
    Direction::West as isize,
];

const BISHOP_DIRS: [isize; 4] = [
    Direction::NorthEast as isize,
    Direction::SouthEast as isize,
    Direction::SouthWest as isize,
    Direction::NorthWest as isize,
];

const fn sliding_attacks<const IS_ROOK: bool>(sq: usize, occ: Bitboard) -> Bitboard {
    let sq = sq as isize;
    let dirs = if IS_ROOK { ROOK_DIRS } else { BISHOP_DIRS };
    let mut i = 0;
    let mut attacks = 0;
    while i < 4 {
        let mut next = sq + dirs[i];
        while is_valid(next) && SQUARES_DISTANCES[next as usize][(next - dirs[i]) as usize] == 1 {
            let bb = Bitboard::from_square(next as usize);
            attacks |= bb.0;
            if occ.0 & bb.0 > 0 {
                break;
            }
            next += dirs[i];
        }
        i += 1;
    }
    Bitboard(attacks)
}

const fn get_relevant_occupancies<const IS_ROOK: bool>() -> [Bitboard; 64] {
    let mut occ = [Bitboard(0); 64];
    let mut sq = 0;
    while sq < 64 {
        let edges = ((RANK_1.0 | RANK_8.0) & !Bitboard::rank(sq).0)
            | ((FILE_A.0 | FILE_H.0) & !Bitboard::file(sq).0);
        occ[sq] = Bitboard(sliding_attacks::<IS_ROOK>(sq, Bitboard(0)).0 & !edges);
        sq += 1;
    }
    occ
}

const ROOK_RELEVANT_OCCUPANCIES: [Bitboard; 64] = get_relevant_occupancies::<true>();
const BISHOP_RELEVANT_OCCUPANCIES: [Bitboard; 64] = get_relevant_occupancies::<false>();

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        0,
        Bitboard(0),
        Bitboard::from_squares([8, 16, 24, 32, 40, 48, 56, 1, 2, 3, 4, 5, 6, 7])
    )]
    #[case(0, Bitboard::from_squares([1, 8]),
           Bitboard::from_squares([1, 8]))]
    fn sliding_attacks_rook_test(
        #[case] sq: usize,
        #[case] occ: Bitboard,
        #[case] expected: Bitboard,
    ) {
        let bb = sliding_attacks::<true>(sq, occ);
        assert_eq!(expected, bb);
    }

    #[rstest]
    #[case(
        0,
        Bitboard(0),
        Bitboard::from_squares([9, 18, 27, 36, 45, 54, 63])
    )]
    #[case(0, Bitboard::from_square(9), Bitboard::from_square(9))]
    fn sliding_attacks_bishop_test(
        #[case] sq: usize,
        #[case] occ: Bitboard,
        #[case] expected: Bitboard,
    ) {
        let bb = sliding_attacks::<false>(sq, occ);
        assert_eq!(expected, bb);
    }

    #[rstest]
    #[case(0, Bitboard::from_squares([1, 2, 3, 4, 5, 6, 8, 16, 24, 32, 40, 48]))]
    #[case(9, Bitboard::from_squares([10, 11, 12, 13, 14, 17, 25, 33, 41, 49]))]
    fn get_relevant_occupancies_rook_test(#[case] sq: usize, #[case] expected: Bitboard) {
        assert_eq!(expected, ROOK_RELEVANT_OCCUPANCIES[sq]);
    }

    #[rstest]
    #[case(0, Bitboard::from_squares([9, 18, 27, 36, 45, 54]))]
    #[case(9, Bitboard::from_squares([18, 27, 36, 45, 54]))]
    fn get_relevant_occupancies_bishop_test(#[case] sq: usize, #[case] expected: Bitboard) {
        assert_eq!(expected, BISHOP_RELEVANT_OCCUPANCIES[sq]);
    }
}
