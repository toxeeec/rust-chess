use crate::chess::{bitboard::shift::Direction, board::Piece, Bitboard, Board};

use super::{
    lookup::{CHECK_PATH, KNIGHT, PIN_PATH},
    magic::{seen_squares_bishop, seen_squares_queen, seen_squares_rook},
};

fn pawn_check<const IS_WHITE: bool>(
    mask: &mut Bitboard,
    board: Board,
    king_sq: usize,
    king_ban: &mut Bitboard,
) {
    let mut pawns_left = if IS_WHITE {
        board.0[Piece::BlackPawn as usize].shifted_forward_left::<false>()
    } else {
        board.0[Piece::WhitePawn as usize].shifted_forward_left::<true>()
    };

    let mut pawns_right = if IS_WHITE {
        board.0[Piece::BlackPawn as usize].shifted_forward_right::<false>()
    } else {
        board.0[Piece::WhitePawn as usize].shifted_forward_right::<true>()
    };

    *king_ban |= pawns_left | pawns_right;

    while pawns_left.0 > 0 {
        let sq = pawns_left.pop_lsb().unwrap();
        if sq == king_sq {
            if IS_WHITE {
                *mask = Bitboard::from_square(sq).shifted(Direction::NorthWest);
            } else {
                *mask = Bitboard::from_square(sq).shifted(Direction::SouthEast);
            }
        }
    }
    while pawns_right.0 > 0 {
        let sq = pawns_right.pop_lsb().unwrap();
        if sq == king_sq {
            if IS_WHITE {
                *mask = Bitboard::from_square(sq).shifted(Direction::NorthEast);
            } else {
                *mask = Bitboard::from_square(sq).shifted(Direction::SouthWest);
            }
        }
    }
}

fn knight_check<const IS_WHITE: bool>(
    mask: &mut Bitboard,
    board: Board,
    king_sq: usize,
    king_ban: &mut Bitboard,
) {
    let mut bb = if IS_WHITE {
        board.0[Piece::BlackKnight as usize]
    } else {
        board.0[Piece::WhiteKnight as usize]
    };
    while bb.0 > 0 {
        let sq = bb.pop_lsb().unwrap();
        let mut attacks = KNIGHT[sq];
        *king_ban |= attacks;
        while attacks.0 > 0 {
            let attack = attacks.pop_lsb().unwrap();
            if attack == king_sq {
                *mask = Bitboard::from_square(sq);
            }
        }
    }
}

fn bishop_check<const IS_WHITE: bool>(
    mask: &mut Bitboard,
    board: Board,
    king_sq: usize,
    king_ban: &mut Bitboard,
) {
    let mut bb = if IS_WHITE {
        board.0[Piece::BlackBishop as usize]
    } else {
        board.0[Piece::WhiteBishop as usize]
    };
    while bb.0 > 0 {
        let sq = bb.pop_lsb().unwrap();
        let mut attacks = seen_squares_bishop(sq, !board.empty());

        *king_ban |= attacks;
        while attacks.0 > 0 {
            let attack = attacks.pop_lsb().unwrap();
            if attack == king_sq {
                *king_ban |= PIN_PATH[king_sq * 64 + sq];
                if *mask == Bitboard(!0) {
                    *mask = CHECK_PATH[king_sq * 64 + sq];
                } else {
                    *mask = Bitboard(0);
                }
            }
        }
    }
}

fn rook_check<const IS_WHITE: bool>(
    mask: &mut Bitboard,
    board: Board,
    king_sq: usize,
    king_ban: &mut Bitboard,
) {
    let mut bb = if IS_WHITE {
        board.0[Piece::BlackRook as usize]
    } else {
        board.0[Piece::WhiteRook as usize]
    };
    while bb.0 > 0 {
        let sq = bb.pop_lsb().unwrap();
        let mut attacks = seen_squares_rook(sq, !board.empty());
        *king_ban |= attacks;
        while attacks.0 > 0 {
            let attack = attacks.pop_lsb().unwrap();
            if attack == king_sq {
                *king_ban |= PIN_PATH[king_sq * 64 + sq];
                if *mask == Bitboard(!0) {
                    *mask = CHECK_PATH[king_sq * 64 + sq];
                } else {
                    *mask = Bitboard(0);
                }
            }
        }
    }
}

fn queen_check<const IS_WHITE: bool>(
    mask: &mut Bitboard,
    board: Board,
    king_sq: usize,
    king_ban: &mut Bitboard,
) {
    let mut bb = if IS_WHITE {
        board.0[Piece::BlackQueen as usize]
    } else {
        board.0[Piece::WhiteQueen as usize]
    };
    while bb.0 > 0 {
        let sq = bb.pop_lsb().unwrap();
        let mut attacks = seen_squares_queen(sq, !board.empty());
        *king_ban |= attacks;
        while attacks.0 > 0 {
            let attack = attacks.pop_lsb().unwrap();
            if attack == king_sq {
                *king_ban |= PIN_PATH[king_sq * 64 + sq];
                if *mask == Bitboard(!0) {
                    *mask = CHECK_PATH[king_sq * 64 + sq];
                } else {
                    *mask = Bitboard(0);
                }
            }
        }
    }
}

//TODO: better lookup (mask is 1 when no possible path, add knight path)?
pub fn checkmask<const IS_WHITE: bool>(board: Board, banned: &mut Bitboard) -> Bitboard {
    let king_sq = if IS_WHITE {
        board.0[Piece::WhiteKing as usize]
    } else {
        board.0[Piece::BlackKing as usize]
    }
    .pop_lsb()
    .unwrap();

    let mut mask = Bitboard(!0);
    pawn_check::<IS_WHITE>(&mut mask, board, king_sq, banned);
    knight_check::<IS_WHITE>(&mut mask, board, king_sq, banned);
    bishop_check::<IS_WHITE>(&mut mask, board, king_sq, banned);
    rook_check::<IS_WHITE>(&mut mask, board, king_sq, banned);
    queen_check::<IS_WHITE>(&mut mask, board, king_sq, banned);

    mask
}

fn diag_pins<const IS_WHITE: bool>(board: Board) -> Bitboard {
    let ally = if IS_WHITE {
        board.enemy::<false>()
    } else {
        board.enemy::<true>()
    };

    let king_bb = if IS_WHITE {
        board.0[Piece::WhiteKing as usize]
    } else {
        board.0[Piece::BlackKing as usize]
    };

    let king_sq = king_bb.get_lsb().unwrap();
    let mut bb = if IS_WHITE {
        board.0[Piece::BlackBishop as usize] | board.0[Piece::BlackQueen as usize]
    } else {
        board.0[Piece::WhiteBishop as usize] | board.0[Piece::WhiteQueen as usize]
    };

    let mut pins = Bitboard(0);

    while bb.0 > 0 {
        let sq = bb.pop_lsb().unwrap();

        let blockers = seen_squares_bishop(sq, ally) & ally;
        let pinned = seen_squares_bishop(sq, !board.empty() & !blockers);

        if pinned & king_bb != Bitboard(0) {
            pins |= CHECK_PATH[king_sq * 64 + sq];
        };
    }

    pins
}

pub fn hv_pins<const IS_WHITE: bool>(board: Board) -> Bitboard {
    let ally = if IS_WHITE {
        board.enemy::<false>()
    } else {
        board.enemy::<true>()
    };

    let king_bb = if IS_WHITE {
        board.0[Piece::WhiteKing as usize]
    } else {
        board.0[Piece::BlackKing as usize]
    };

    let king_sq = king_bb.get_lsb().unwrap();
    let mut rooks = if IS_WHITE {
        board.0[Piece::BlackRook as usize] | board.0[Piece::BlackQueen as usize]
    } else {
        board.0[Piece::WhiteRook as usize] | board.0[Piece::WhiteQueen as usize]
    };

    let mut pins = Bitboard(0);

    while rooks.0 > 0 {
        let sq = rooks.pop_lsb().unwrap();

        let blockers = seen_squares_rook(sq, ally) & ally;
        let pinned = seen_squares_rook(sq, !board.empty() & !blockers);

        if pinned & king_bb != Bitboard(0) {
            pins |= CHECK_PATH[king_sq * 64 + sq];
        };
    }

    pins
}

#[derive(Debug, Clone, Copy)]
pub struct Pins {
    pub hv: Bitboard,
    pub diag: Bitboard,
}

pub fn pinmask<const IS_WHITE: bool>(board: Board) -> Pins {
    let hv_pins = hv_pins::<IS_WHITE>(board);
    let diagonal_pins = diag_pins::<IS_WHITE>(board);

    Pins {
        hv: hv_pins,
        diag: diagonal_pins,
    }
}
