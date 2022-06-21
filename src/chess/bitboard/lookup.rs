use super::{shift::KNIGHTDIR_ITEMS, Bitboard};

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

pub const KNIGHT: [Bitboard; 64] = knight();
