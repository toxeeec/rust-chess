use crate::chess::{board::Piece, state::State, Bitboard, Board};

use super::{
    lookup::{KING, KNIGHT},
    magic::{seen_squares_bishop, seen_squares_queen, seen_squares_rook},
    masks::Pins,
    r#type::Flag,
    List,
};

const WHITE_CASTLE_LEFT_PATH: Bitboard = Bitboard::from_squares([2, 3, 4]);
const WHITE_CASTLE_LEFT_BETWEEN: Bitboard = Bitboard::from_squares([1, 2, 3]);

const WHITE_CASTLE_RIGHT_PATH: Bitboard = Bitboard::from_squares([4, 5, 6]);
const WHITE_CASTLE_RIGHT_BETWEEN: Bitboard = Bitboard::from_squares([5, 6]);

const BLACK_CASTLE_LEFT_PATH: Bitboard = Bitboard::from_squares([58, 59, 60]);
const BLACK_CASTLE_LEFT_BETWEEN: Bitboard = Bitboard::from_squares([57, 58, 59]);

const BLACK_CASTLE_RIGHT_PATH: Bitboard = Bitboard::from_squares([60, 61, 62]);
const BLACK_CASTLE_RIGHT_BETWEEN: Bitboard = Bitboard::from_squares([61, 62]);

const WHITE_LEFT_ROOK_BB: Bitboard = Bitboard::from_square(0);
const WHITE_RIGHT_ROOK_BB: Bitboard = Bitboard::from_square(7);
const BLACK_LEFT_ROOK_BB: Bitboard = Bitboard::from_square(56);
const BLACK_RIGHT_ROOK_BB: Bitboard = Bitboard::from_square(63);

pub fn is_left_rook<const IS_WHITE: bool>(bb: Bitboard) -> bool {
    if IS_WHITE {
        (WHITE_LEFT_ROOK_BB & bb).0 > 0
    } else {
        (BLACK_LEFT_ROOK_BB & bb).0 > 0
    }
}

pub fn is_right_rook<const IS_WHITE: bool>(bb: Bitboard) -> bool {
    if IS_WHITE {
        (WHITE_RIGHT_ROOK_BB & bb).0 > 0
    } else {
        (BLACK_RIGHT_ROOK_BB & bb).0 > 0
    }
}

fn can_castle_left<const IS_WHITE: bool>(board: Board, banned: Bitboard) -> bool {
    if IS_WHITE {
        if (WHITE_CASTLE_LEFT_BETWEEN & !board.empty()).0 > 0
            || (WHITE_CASTLE_LEFT_PATH & banned).0 > 0
        {
            false
        } else {
            is_left_rook::<IS_WHITE>(board.0[Piece::WhiteRook as usize])
        }
    } else if (BLACK_CASTLE_LEFT_BETWEEN & !board.empty()).0 > 0
        || (BLACK_CASTLE_LEFT_PATH & banned).0 > 0
    {
        false
    } else {
        is_left_rook::<IS_WHITE>(board.0[Piece::BlackRook as usize])
    }
}

fn can_castle_right<const IS_WHITE: bool>(board: Board, banned: Bitboard) -> bool {
    if IS_WHITE {
        if (WHITE_CASTLE_RIGHT_BETWEEN & !board.empty()).0 > 0
            || (WHITE_CASTLE_RIGHT_PATH & banned).0 > 0
        {
            false
        } else {
            is_right_rook::<IS_WHITE>(board.0[Piece::WhiteRook as usize])
        }
    } else if (BLACK_CASTLE_RIGHT_BETWEEN & !board.empty()).0 > 0
        || (BLACK_CASTLE_RIGHT_PATH & banned).0 > 0
    {
        false
    } else {
        is_right_rook::<IS_WHITE>(board.0[Piece::BlackRook as usize])
    }
}

fn add_castle_left<const IS_WHITE: bool>(
    list: &mut List,
    board: Board,
    state: State,
    banned: Bitboard,
) {
    if IS_WHITE {
        if !state.can_castle_wl {
            return;
        }
    } else if !state.can_castle_bl {
        return;
    }

    if !can_castle_left::<IS_WHITE>(board, banned) {
        return;
    }

    if IS_WHITE {
        list.add(4, 2, Flag::QueenCastle)
    } else {
        list.add(60, 58, Flag::QueenCastle)
    }
}
fn add_castle_right<const IS_WHITE: bool>(
    list: &mut List,
    board: Board,
    state: State,
    banned: Bitboard,
) {
    if IS_WHITE {
        if !state.can_castle_wr {
            return;
        }
    } else if !state.can_castle_br {
        return;
    }

    if !can_castle_right::<IS_WHITE>(board, banned) {
        return;
    }

    if IS_WHITE {
        list.add(4, 6, Flag::KingCastle)
    } else {
        list.add(60, 62, Flag::KingCastle)
    }
}

impl List {
    pub fn add_king_moves<const IS_WHITE: bool>(
        &mut self,
        board: Board,
        state: State,
        banned: Bitboard,
    ) {
        let mut bb = if IS_WHITE {
            board.0[Piece::WhiteKing as usize]
        } else {
            board.0[Piece::BlackKing as usize]
        };

        let from = bb.pop_lsb().unwrap();
        let mut moves = KING[from] & board.empty() & !banned;
        while moves.0 > 0 {
            let to = moves.pop_lsb().unwrap();
            self.add(from, to, Flag::Quiet);
        }

        let mut captures = KING[from] & board.enemy::<IS_WHITE>() & !banned;
        while captures.0 > 0 {
            let to = captures.pop_lsb().unwrap();
            self.add(from, to, Flag::Capture);
        }

        add_castle_left::<IS_WHITE>(self, board, state, banned);
        add_castle_right::<IS_WHITE>(self, board, state, banned);
    }

    pub fn add_knight_moves<const IS_WHITE: bool>(
        &mut self,
        board: Board,
        checkmask: Bitboard,
        pins: Pins,
    ) {
        let mut bb = if IS_WHITE {
            board.0[Piece::WhiteKnight as usize]
        } else {
            board.0[Piece::BlackKnight as usize]
        } & !(pins.hv | pins.diag);

        while bb.0 > 0 {
            let from = bb.pop_lsb().unwrap();
            let mut moves = KNIGHT[from] & board.empty() & checkmask;
            while moves.0 > 0 {
                let to = moves.pop_lsb().unwrap();
                self.add(from, to, Flag::Quiet);
            }

            let mut captures = KNIGHT[from] & board.enemy::<IS_WHITE>() & checkmask;
            while captures.0 > 0 {
                let to = captures.pop_lsb().unwrap();
                self.add(from, to, Flag::Capture);
            }
        }
    }

    pub fn add_bishop_moves<const IS_WHITE: bool>(
        &mut self,
        board: Board,
        checkmask: Bitboard,
        pins: Pins,
    ) {
        let bb = if IS_WHITE {
            board.0[Piece::WhiteBishop as usize]
        } else {
            board.0[Piece::BlackBishop as usize]
        } & !pins.hv;

        let mut pinned = bb & pins.diag;
        let mut not_pinned = bb & !pins.diag;

        while pinned.0 > 0 {
            let from = pinned.pop_lsb().unwrap();
            let seen_squares = seen_squares_bishop(from, !board.empty()) & pins.diag;
            let mut moves = seen_squares & board.empty() & checkmask;
            while moves.0 > 0 {
                let to = moves.pop_lsb().unwrap();
                self.add(from, to, Flag::Quiet);
            }

            let mut captures = seen_squares & board.enemy::<IS_WHITE>() & checkmask;
            while captures.0 > 0 {
                let to = captures.pop_lsb().unwrap();
                self.add(from, to, Flag::Capture);
            }
        }

        while not_pinned.0 > 0 {
            let from = not_pinned.pop_lsb().unwrap();
            let seen_squares = seen_squares_bishop(from, !board.empty());
            let mut moves = seen_squares & board.empty() & checkmask;
            while moves.0 > 0 {
                let to = moves.pop_lsb().unwrap();
                self.add(from, to, Flag::Quiet);
            }

            let mut captures = seen_squares & board.enemy::<IS_WHITE>() & checkmask;
            while captures.0 > 0 {
                let to = captures.pop_lsb().unwrap();
                self.add(from, to, Flag::Capture);
            }
        }
    }

    pub fn add_rook_moves<const IS_WHITE: bool>(
        &mut self,
        board: Board,
        checkmask: Bitboard,
        pins: Pins,
    ) {
        let bb = if IS_WHITE {
            board.0[Piece::WhiteRook as usize]
        } else {
            board.0[Piece::BlackRook as usize]
        } & !pins.diag;

        let mut pinned = bb & pins.hv;
        let mut not_pinned = bb & !pins.hv;

        while pinned.0 > 0 {
            let from = pinned.pop_lsb().unwrap();
            let seen_squares = seen_squares_rook(from, !board.empty()) & pins.hv;
            let mut moves = seen_squares & board.empty() & checkmask;
            while moves.0 > 0 {
                let to = moves.pop_lsb().unwrap();
                self.add(from, to, Flag::Quiet);
            }

            let mut captures = seen_squares & board.enemy::<IS_WHITE>() & checkmask;
            while captures.0 > 0 {
                let to = captures.pop_lsb().unwrap();
                self.add(from, to, Flag::Capture);
            }
        }

        while not_pinned.0 > 0 {
            let from = not_pinned.pop_lsb().unwrap();
            let seen_squares = seen_squares_rook(from, !board.empty());
            let mut moves = seen_squares & board.empty() & checkmask;
            while moves.0 > 0 {
                let to = moves.pop_lsb().unwrap();
                self.add(from, to, Flag::Quiet);
            }

            let mut captures = seen_squares & board.enemy::<IS_WHITE>() & checkmask;
            while captures.0 > 0 {
                let to = captures.pop_lsb().unwrap();
                self.add(from, to, Flag::Capture);
            }
        }
    }

    pub fn add_queen_moves<const IS_WHITE: bool>(
        &mut self,
        board: Board,
        checkmask: Bitboard,
        pins: Pins,
    ) {
        let bb = if IS_WHITE {
            board.0[Piece::WhiteQueen as usize]
        } else {
            board.0[Piece::BlackQueen as usize]
        };

        let mut hv_pinned = bb & pins.hv;
        let mut diag_pinned = bb & pins.diag;
        let mut not_pinned = bb & !(pins.diag | pins.hv);

        while hv_pinned.0 > 0 {
            let from = hv_pinned.pop_lsb().unwrap();
            let seen_squares = seen_squares_rook(from, !board.empty()) & pins.hv;
            let mut moves = seen_squares & board.empty() & checkmask;
            while moves.0 > 0 {
                let to = moves.pop_lsb().unwrap();
                self.add(from, to, Flag::Quiet);
            }

            let mut captures = seen_squares & board.enemy::<IS_WHITE>() & checkmask;
            while captures.0 > 0 {
                let to = captures.pop_lsb().unwrap();
                self.add(from, to, Flag::Capture);
            }
        }

        while diag_pinned.0 > 0 {
            let from = diag_pinned.pop_lsb().unwrap();
            let seen_squares = seen_squares_bishop(from, !board.empty()) & pins.diag;
            let mut moves = seen_squares & board.empty() & checkmask;
            while moves.0 > 0 {
                let to = moves.pop_lsb().unwrap();
                self.add(from, to, Flag::Quiet);
            }

            let mut captures = seen_squares & board.enemy::<IS_WHITE>() & checkmask;
            while captures.0 > 0 {
                let to = captures.pop_lsb().unwrap();
                self.add(from, to, Flag::Capture);
            }
        }

        while not_pinned.0 > 0 {
            let from = not_pinned.pop_lsb().unwrap();
            let seen_squares = seen_squares_queen(from, !board.empty());
            let mut moves = seen_squares & board.empty() & checkmask;
            while moves.0 > 0 {
                let to = moves.pop_lsb().unwrap();
                self.add(from, to, Flag::Quiet);
            }

            let mut captures = seen_squares & board.enemy::<IS_WHITE>() & checkmask;
            while captures.0 > 0 {
                let to = captures.pop_lsb().unwrap();
                self.add(from, to, Flag::Capture);
            }
        }
    }
}
