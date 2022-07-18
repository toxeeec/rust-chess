use crate::chess::{
    bitboard::{shift::Direction, RANK_1, RANK_3, RANK_6, RANK_8},
    board::Piece,
    Bitboard, Board,
};

use super::{list::List, r#type::Flag};

const fn last_rank<const IS_WHITE: bool>() -> Bitboard {
    if IS_WHITE {
        RANK_8
    } else {
        RANK_1
    }
}

const fn third_rank<const IS_WHITE: bool>() -> Bitboard {
    if IS_WHITE {
        RANK_3
    } else {
        RANK_6
    }
}

fn add_single_pushes<const IS_WHITE: bool>(mut bb: Bitboard, list: &mut List) {
    while bb.0 > 0 {
        let to = bb.pop_lsb().unwrap();
        let from = if IS_WHITE {
            to - Direction::North as usize
        } else {
            to + Direction::North as usize
        };
        list.add(from, to, Flag::Quiet);
    }
}

fn add_double_pushes<const IS_WHITE: bool>(mut bb: Bitboard, list: &mut List) {
    while bb.0 > 0 {
        let to = bb.pop_lsb().unwrap();
        let from = if IS_WHITE {
            to - (Direction::North as usize) * 2
        } else {
            to + (Direction::North as usize) * 2
        };
        list.add(from, to, Flag::DoublePush);
    }
}

fn add_captures<const IS_WHITE: bool, const IS_LEFT: bool>(mut bb: Bitboard, list: &mut List) {
    let dir = if IS_LEFT {
        Direction::NorthWest
    } else {
        Direction::NorthEast
    };
    while bb.0 > 0 {
        let to = bb.pop_lsb().unwrap();
        let from = if IS_WHITE {
            to - dir as usize
        } else {
            to + dir as usize
        };
        list.add(from, to, Flag::Capture);
    }
}

impl List {
    pub fn add_pawn_moves<const IS_WHITE: bool>(&mut self, board: Board, checkmask: Bitboard) {
        let bb = if IS_WHITE {
            board.0[Piece::WhitePawn as usize]
        } else {
            board.0[Piece::BlackPawn as usize]
        };
        let mut pushed = bb.shifted_forward::<IS_WHITE>();
        pushed &= !last_rank::<IS_WHITE>();
        pushed &= board.empty();
        let mut double_pushed = pushed;
        pushed &= checkmask;
        add_single_pushes::<IS_WHITE>(pushed, self);

        double_pushed &= third_rank::<IS_WHITE>();
        double_pushed = double_pushed.shifted_forward::<IS_WHITE>();
        double_pushed &= board.empty();
        double_pushed &= checkmask;
        add_double_pushes::<IS_WHITE>(double_pushed, self);

        let mut shifted = bb.shifted_forward_left::<IS_WHITE>();
        shifted &= !last_rank::<IS_WHITE>();
        shifted &= board.enemy::<IS_WHITE>();
        shifted &= checkmask;
        add_captures::<IS_WHITE, true>(shifted, self);

        shifted = bb.shifted_forward_right::<IS_WHITE>();
        shifted &= !last_rank::<IS_WHITE>();
        shifted &= board.enemy::<IS_WHITE>();
        shifted &= checkmask;
        add_captures::<IS_WHITE, false>(shifted, self);

        //TODO: en passant, promotions, promotion captures, checks, pins
    }
}
