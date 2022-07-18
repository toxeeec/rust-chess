use crate::chess::{bitboard::shift::Direction, board::Piece, Bitboard, Board};

use super::{
    lookup::{CHECK_PATH, KING, KNIGHT, PIN_PATH},
    magic::{seen_squares_bishop, seen_squares_queen, seen_squares_rook},
};

fn pawn_check<const IS_WHITE: bool>(
    mask: &mut Bitboard,
    board: Board,
    king_sq: usize,
    king_ban: &mut Bitboard,
) -> bool {
    let mut pawns_left = if IS_WHITE {
        board.0[Piece::BlackPawn as usize].shifted_forward_left::<IS_WHITE>()
    } else {
        board.0[Piece::WhitePawn as usize].shifted_forward_left::<IS_WHITE>()
    };

    let mut pawns_right = if IS_WHITE {
        board.0[Piece::BlackPawn as usize].shifted_forward_right::<IS_WHITE>()
    } else {
        board.0[Piece::WhitePawn as usize].shifted_forward_right::<IS_WHITE>()
    };

    *king_ban |= pawns_left | pawns_right;

    let dir = Direction::NorthWest as usize;
    while pawns_left.0 > 0 {
        let sq = pawns_left.pop_lsb().unwrap();
        if sq == king_sq {
            if IS_WHITE {
                *mask = Bitboard::from_square(sq - dir);
            } else {
                *mask = Bitboard::from_square(sq + dir);
            }
            return true;
        }
    }
    let dir = Direction::NorthEast as usize;
    while pawns_right.0 > 0 {
        let sq = pawns_right.pop_lsb().unwrap();
        if sq == king_sq {
            if IS_WHITE {
                *mask = Bitboard::from_square(sq - dir);
            } else {
                *mask = Bitboard::from_square(sq + dir);
            }
            return true;
        }
    }
    false
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
                return;
            }
        }
    }
}

fn bishop_check<const IS_WHITE: bool>(
    mask: &mut Bitboard,
    board: Board,
    king_sq: usize,
    king_ban: &mut Bitboard,
) -> bool {
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
                    return true;
                }
            }
        }
    }
    false
}

fn rook_check<const IS_WHITE: bool>(
    mask: &mut Bitboard,
    board: Board,
    king_sq: usize,
    king_ban: &mut Bitboard,
) -> bool {
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
                    return true;
                }
            }
        }
    }
    false
}

fn queen_check<const IS_WHITE: bool>(
    mask: &mut Bitboard,
    board: Board,
    king_sq: usize,
    king_ban: &mut Bitboard,
) -> bool {
    let mut bb = if IS_WHITE {
        board.0[Piece::BlackQueen as usize]
    } else {
        board.0[Piece::WhiteQueen as usize]
    };
    while bb.0 > 0 {
        let sq = bb.pop_lsb().unwrap();
        let mut attacks = seen_squares_queen(sq, !board.empty());
        while attacks.0 > 0 {
            let attack = attacks.pop_lsb().unwrap();
            if attack == king_sq {
                *king_ban |= PIN_PATH[king_sq * 64 + sq];
                if *mask == Bitboard(!0) {
                    *mask = CHECK_PATH[king_sq * 64 + sq];
                } else {
                    *mask = Bitboard(0);
                    return true;
                }
            }
        }
    }
    false
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
    let checked = pawn_check::<IS_WHITE>(&mut mask, board, king_sq, banned);
    if checked {
        return mask;
    }
    knight_check::<IS_WHITE>(&mut mask, board, king_sq, banned);
    let mut double_check = bishop_check::<IS_WHITE>(&mut mask, board, king_sq, banned);
    if double_check {
        return mask;
    }
    double_check = rook_check::<IS_WHITE>(&mut mask, board, king_sq, banned);
    if double_check {
        return mask;
    }
    queen_check::<IS_WHITE>(&mut mask, board, king_sq, banned);

    mask
}