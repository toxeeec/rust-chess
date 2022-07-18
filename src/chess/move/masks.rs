use crate::chess::{bitboard::shift::Direction, board::Piece, Bitboard, Board};

use super::{
    lookup::{CHECK_PATH, KING, KNIGHT},
    magic::{seen_squares_bishop, seen_squares_queen, seen_squares_rook},
};

fn pawn_check<const IS_WHITE: bool>(mask: &mut Bitboard, board: Board, king_sq: usize) -> bool {
    let mut pawns_left = if IS_WHITE {
        board.0[Piece::BlackPawn as usize].shifted_forward_left::<IS_WHITE>()
    } else {
        board.0[Piece::WhitePawn as usize].shifted_forward_left::<IS_WHITE>()
    };
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
    let mut pawns_right = if IS_WHITE {
        board.0[Piece::BlackPawn as usize].shifted_forward_right::<IS_WHITE>()
    } else {
        board.0[Piece::WhitePawn as usize].shifted_forward_right::<IS_WHITE>()
    };
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

fn knight_check<const IS_WHITE: bool>(mask: &mut Bitboard, board: Board, king_sq: usize) {
    let mut bb = if IS_WHITE {
        board.0[Piece::BlackKnight as usize]
    } else {
        board.0[Piece::WhiteKnight as usize]
    };
    while bb.0 > 0 {
        let sq = bb.pop_lsb().unwrap();
        let mut attacks = KNIGHT[sq];
        while attacks.0 > 0 {
            let attack = attacks.pop_lsb().unwrap();
            if attack == king_sq {
                *mask = Bitboard::from_square(sq);
                return;
            }
        }
    }
}

fn bishop_check<const IS_WHITE: bool>(mask: &mut Bitboard, board: Board, king_sq: usize) -> bool {
    let mut bb = if IS_WHITE {
        board.0[Piece::BlackBishop as usize]
    } else {
        board.0[Piece::WhiteBishop as usize]
    };
    while bb.0 > 0 {
        let sq = bb.pop_lsb().unwrap();
        let mut attacks = seen_squares_bishop(sq, !board.empty());
        while attacks.0 > 0 {
            let attack = attacks.pop_lsb().unwrap();
            if attack == king_sq {
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

fn rook_check<const IS_WHITE: bool>(mask: &mut Bitboard, board: Board, king_sq: usize) -> bool {
    let mut bb = if IS_WHITE {
        board.0[Piece::BlackRook as usize]
    } else {
        board.0[Piece::WhiteRook as usize]
    };
    while bb.0 > 0 {
        let sq = bb.pop_lsb().unwrap();
        let mut attacks = seen_squares_rook(sq, !board.empty());
        while attacks.0 > 0 {
            let attack = attacks.pop_lsb().unwrap();
            if attack == king_sq {
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

fn queen_check<const IS_WHITE: bool>(mask: &mut Bitboard, board: Board, king_sq: usize) -> bool {
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

pub fn checkmask<const IS_WHITE: bool>(board: Board) -> Bitboard {
    let king_sq = if IS_WHITE {
        board.0[Piece::WhiteKing as usize]
    } else {
        board.0[Piece::BlackKing as usize]
    }
    .pop_lsb()
    .unwrap();

    let mut mask = Bitboard(!0);
    let checked = pawn_check::<IS_WHITE>(&mut mask, board, king_sq);
    if checked {
        return mask;
    }
    knight_check::<IS_WHITE>(&mut mask, board, king_sq);
    let mut double_check = bishop_check::<IS_WHITE>(&mut mask, board, king_sq);
    if double_check {
        return mask;
    }
    double_check = rook_check::<IS_WHITE>(&mut mask, board, king_sq);
    if double_check {
        return mask;
    }
    queen_check::<IS_WHITE>(&mut mask, board, king_sq);

    mask
}

pub fn seen_squares_enemy<const IS_WHITE: bool>(board: Board) -> Bitboard {
    let mut seen_squares = Bitboard(0);
    let king_sq = if IS_WHITE {
        board.0[Piece::BlackKing as usize]
    } else {
        board.0[Piece::WhiteKing as usize]
    }
    .pop_lsb()
    .unwrap();
    seen_squares |= KING[king_sq];
    let pawns = if IS_WHITE {
        board.0[Piece::BlackPawn as usize]
    } else {
        board.0[Piece::WhitePawn as usize]
    };

    seen_squares |= if IS_WHITE {
        pawns.shifted_forward_left::<false>() | pawns.shifted_forward_right::<false>()
    } else {
        pawns.shifted_forward_left::<true>() | pawns.shifted_forward_right::<false>()
    };

    let mut knights = if IS_WHITE {
        board.0[Piece::BlackKnight as usize]
    } else {
        board.0[Piece::WhiteKnight as usize]
    };
    while knights.0 > 0 {
        let sq = knights.pop_lsb().unwrap();
        seen_squares |= KNIGHT[sq];
    }

    let mut rooks = if IS_WHITE {
        board.0[Piece::BlackRook as usize]
    } else {
        board.0[Piece::WhiteRook as usize]
    };
    while rooks.0 > 0 {
        let sq = rooks.pop_lsb().unwrap();
        seen_squares |= seen_squares_rook(sq, !board.empty());
    }

    let mut bishops = if IS_WHITE {
        board.0[Piece::BlackBishop as usize]
    } else {
        board.0[Piece::WhiteBishop as usize]
    };
    while bishops.0 > 0 {
        let sq = bishops.pop_lsb().unwrap();
        seen_squares |= seen_squares_bishop(sq, !board.empty());
    }

    let mut queens = if IS_WHITE {
        board.0[Piece::BlackQueen as usize]
    } else {
        board.0[Piece::WhiteQueen as usize]
    };
    while queens.0 > 0 {
        let sq = queens.pop_lsb().unwrap();
        seen_squares |= seen_squares_queen(sq, !board.empty());
    }
    seen_squares
}
