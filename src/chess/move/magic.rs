use crate::chess::{
    bitboard::{shift::Direction, FILE_A, FILE_H, RANK_1, RANK_8},
    square::SQUARES_DISTANCES,
    Bitboard,
};
use lazy_static::lazy_static;
use rand::Rng;

const ROOK_DIRS: [usize; 4] = [
    Direction::North as usize,
    Direction::East as usize,
    Direction::South as usize,
    Direction::West as usize,
];

const BISHOP_DIRS: [usize; 4] = [
    Direction::NorthEast as usize,
    Direction::SouthEast as usize,
    Direction::SouthWest as usize,
    Direction::NorthWest as usize,
];

const fn sliding_attacks<const IS_ROOK: bool>(sq: usize, occ: Bitboard) -> Bitboard {
    let dirs = if IS_ROOK { ROOK_DIRS } else { BISHOP_DIRS };
    let mut i = 0;
    let mut attacks = 0;
    while i < 4 {
        let mut next = sq.wrapping_add(dirs[i]);
        while next < 64 && SQUARES_DISTANCES[next][(next.wrapping_sub(dirs[i]))] == 1 {
            let bb = Bitboard::from_square(next);
            attacks |= bb.0;
            if occ.0 & bb.0 > 0 {
                break;
            }
            next = next.wrapping_add(dirs[i]);
        }
        i += 1;
    }
    Bitboard(attacks)
}

const fn relevant_occupancies<const IS_ROOK: bool>() -> [Bitboard; 64] {
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

const ROOK_RELEVANT_OCCUPANCIES: [Bitboard; 64] = relevant_occupancies::<true>();
const BISHOP_RELEVANT_OCCUPANCIES: [Bitboard; 64] = relevant_occupancies::<false>();

#[derive(Default, Debug)]
pub struct Magic {
    pub mask: Bitboard,
    pub magic: u64,
    pub shift: u32,
    pub attacks: Vec<Bitboard>,
}

//TODO: move to external crate
fn magics<const IS_ROOK: bool>() -> Vec<Magic> {
    let mut rng = rand::thread_rng();
    let mut magics: Vec<Magic> = Vec::with_capacity(64);
    let mut blockers = [Bitboard(0); 4096];
    let mut reference = [Bitboard(0); 4096];
    for sq in 0..64 {
        let mask = if IS_ROOK {
            ROOK_RELEVANT_OCCUPANCIES[sq]
        } else {
            BISHOP_RELEVANT_OCCUPANCIES[sq]
        };
        let shift = mask.0.count_ones();
        let mut size = 0;
        let mut bb = 0;
        loop {
            blockers[size] = Bitboard(bb);
            reference[size] = sliding_attacks::<IS_ROOK>(sq, Bitboard(bb));
            size += 1;
            bb = bb.wrapping_sub(mask.0) & mask.0;
            if bb == 0 {
                break;
            }
        }

        let mut attacks = vec![Bitboard(0); size];

        let mut magic;
        loop {
            magic = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
            attacks.fill(Bitboard(0));
            let mut i = 0;
            while i < size {
                let index = blockers[i].0.wrapping_mul(magic).wrapping_shr(64 - shift);
                let attack = attacks[index as usize];
                if attack.0 > 0 && attack.0 != reference[i].0 {
                    break;
                }
                attacks[index as usize] = reference[i];
                i += 1;
            }
            if i == size {
                break;
            }
        }
        magics.push(Magic {
            mask,
            magic,
            shift,
            attacks,
        });
    }
    magics
}

lazy_static! {
    static ref ROOK_MAGICS: Vec<Magic> = magics::<true>();
    static ref BISHOP_MAGICS: Vec<Magic> = magics::<false>();
}

pub fn seen_squares_rook(sq: usize, blockers: Bitboard) -> Bitboard {
    let index = (blockers.0 & ROOK_MAGICS[sq].mask.0).wrapping_mul(ROOK_MAGICS[sq].magic)
        >> (64 - ROOK_MAGICS[sq].shift);
    ROOK_MAGICS[sq].attacks[index as usize]
}

pub fn seen_squares_bishop(sq: usize, blockers: Bitboard) -> Bitboard {
    let index = (blockers.0 & BISHOP_MAGICS[sq].mask.0).wrapping_mul(BISHOP_MAGICS[sq].magic)
        >> (64 - BISHOP_MAGICS[sq].shift);
    BISHOP_MAGICS[sq].attacks[index as usize]
}

pub fn seen_squares_queen(sq: usize, blockers: Bitboard) -> Bitboard {
    seen_squares_rook(sq, blockers) | seen_squares_bishop(sq, blockers)
}

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
    #[case(0, Bitboard::from_squares([1, 8]), Bitboard::from_squares([1, 8]))]
    fn sliding_attacks_rook_test(
        #[case] sq: usize,
        #[case] occ: Bitboard,
        #[case] expected: Bitboard,
    ) {
        let bb = sliding_attacks::<true>(sq, occ);
        assert_eq!(expected, bb);
    }

    #[rstest]
    #[case(0, Bitboard(0), Bitboard::from_squares([9, 18, 27, 36, 45, 54, 63]))]
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

    #[rstest]
    #[case(0, Bitboard(0), Bitboard::from_squares([1, 2, 3, 4, 5, 6, 7, 8, 16, 24, 32, 40, 48, 56]))]
    #[case(0, Bitboard::from_square(8), Bitboard::from_squares([8, 1, 2, 3, 4, 5, 6, 7]))]
    #[case(0, Bitboard::from_squares([2, 16, 18]), Bitboard::from_squares([1, 2, 8, 16]))]
    fn seen_squares_rook_test(
        #[case] sq: usize,
        #[case] blockers: Bitboard,
        #[case] expected: Bitboard,
    ) {
        assert_eq!(expected, seen_squares_rook(sq, blockers));
    }

    #[rstest]
    #[case(9, Bitboard(0), Bitboard::from_squares([0, 2, 16, 18, 27, 36, 45, 54, 63]))]
    #[case(9, Bitboard::from_square(18), Bitboard::from_squares([0, 2, 16, 18]))]
    #[case(0, Bitboard::from_squares([2, 16, 18]), Bitboard::from_squares([9, 18]))]
    fn seen_squares_bishop_test(
        #[case] sq: usize,
        #[case] blockers: Bitboard,
        #[case] expected: Bitboard,
    ) {
        assert_eq!(expected, seen_squares_bishop(sq, blockers));
    }

    #[rstest]
    #[case(0, Bitboard(0), Bitboard::from_squares([1, 2, 3, 4, 5, 6, 7, 8, 16, 24, 32, 40, 48, 56, 9, 18, 27, 36, 45, 54, 63]))]
    #[case(0, Bitboard::from_squares([2, 16, 18]), Bitboard::from_squares([8, 16, 9, 18, 1, 2]))]
    fn seen_squares_queen_test(
        #[case] sq: usize,
        #[case] blockers: Bitboard,
        #[case] expected: Bitboard,
    ) {
        println!("{}\n", Bitboard::from_squares([2, 16, 18]));
        println!("{}\n", expected);
        println!("{}\n", seen_squares_rook(sq, blockers));
        println!("{}\n", seen_squares_bishop(sq, blockers));
        println!("{}\n", seen_squares_queen(sq, blockers));
        assert_eq!(expected, seen_squares_queen(sq, blockers));
    }
}
