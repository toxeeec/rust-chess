use crate::chess::{board::Piece, state::State, Bitboard, Board};

use super::{
    lookup::{KING, KNIGHT},
    magic::{seen_squares_bishop, seen_squares_queen, seen_squares_rook},
    masks::Pins,
    r#type::Flag,
    List,
};

const WHITE_CASTLE_LEFT_PATH: Bitboard = Bitboard::from_squares([2, 3]);
const WHITE_CASTLE_RIGHT_PATH: Bitboard = Bitboard::from_squares([5, 6]);
const BLACK_CASTLE_LEFT_PATH: Bitboard = Bitboard::from_squares([58, 59]);
const BLACK_CASTLE_RIGHT_PATH: Bitboard = Bitboard::from_squares([61, 62]);

fn add_castle_left<const IS_WHITE: bool>(list: &mut List, board: Board, state: State) {
    if IS_WHITE {
        if !state.can_castle_wl {
            return;
        }
    } else if !state.can_castle_bl {
        return;
    }

    if IS_WHITE {
        if WHITE_CASTLE_LEFT_PATH & board.empty() != WHITE_CASTLE_LEFT_PATH {
            return;
        }
    } else if BLACK_CASTLE_LEFT_PATH & board.empty() != BLACK_CASTLE_LEFT_PATH {
        return;
    }
    //TODO: check if legal
    if IS_WHITE {
        list.add(4, 2, Flag::QueenCastle)
    } else {
        list.add(60, 58, Flag::QueenCastle)
    }
}
fn add_castle_right<const IS_WHITE: bool>(list: &mut List, board: Board, state: State) {
    if IS_WHITE {
        if !state.can_castle_wr {
            return;
        }
    } else if !state.can_castle_br {
        return;
    }
    if IS_WHITE {
        if WHITE_CASTLE_RIGHT_PATH & board.empty() != WHITE_CASTLE_RIGHT_PATH {
            return;
        }
    } else if BLACK_CASTLE_RIGHT_PATH & board.empty() != BLACK_CASTLE_RIGHT_PATH {
        return;
    }
    //TODO: check if legal
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

        let mut captures = KNIGHT[from] & board.enemy::<IS_WHITE>();
        while captures.0 > 0 {
            let to = captures.pop_lsb().unwrap();
            self.add(from, to, Flag::Capture);
        }

        add_castle_left::<IS_WHITE>(self, board, state);
        add_castle_right::<IS_WHITE>(self, board, state);
        //TODO: check
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
        };

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
        //TODO: checks, pins
    }

    pub fn add_bishop_moves<const IS_WHITE: bool>(
        &mut self,
        board: Board,
        checkmask: Bitboard,
        pins: Pins,
    ) {
        let mut bb = if IS_WHITE {
            board.0[Piece::WhiteBishop as usize]
        } else {
            board.0[Piece::BlackBishop as usize]
        };

        while bb.0 > 0 {
            let from = bb.pop_lsb().unwrap();
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
        //TODO: checks, pins
    }

    pub fn add_rook_moves<const IS_WHITE: bool>(
        &mut self,
        board: Board,
        checkmask: Bitboard,
        pins: Pins,
    ) {
        let mut bb = if IS_WHITE {
            board.0[Piece::WhiteRook as usize]
        } else {
            board.0[Piece::BlackRook as usize]
        };

        while bb.0 > 0 {
            let from = bb.pop_lsb().unwrap();
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
        //TODO: checks, pins
    }

    pub fn add_queen_moves<const IS_WHITE: bool>(
        &mut self,
        board: Board,
        checkmask: Bitboard,
        pins: Pins,
    ) {
        let mut bb = if IS_WHITE {
            board.0[Piece::WhiteQueen as usize]
        } else {
            board.0[Piece::BlackQueen as usize]
        };

        while bb.0 > 0 {
            let from = bb.pop_lsb().unwrap();
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
        //TODO: checks, pins
    }
}
