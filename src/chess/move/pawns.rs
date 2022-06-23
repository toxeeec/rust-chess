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

fn add_single_pushes<const IS_WHITE: bool>(mut pushed: Bitboard, list: &mut List) {
    while pushed.0 > 0 {
        let to = pushed.pop_lsb().unwrap();
        let from: usize;
        if IS_WHITE {
            from = to - Direction::North as usize;
        } else {
            from = to + Direction::North as usize;
        }
        list.add(from, to, Flag::Quiet);
    }
}

fn add_double_pushes<const IS_WHITE: bool>(mut pushed: Bitboard, list: &mut List) {
    while pushed.0 > 0 {
        let to = pushed.pop_lsb().unwrap();
        let from: usize;
        if IS_WHITE {
            from = to - (Direction::North as usize) * 2;
        } else {
            from = to + (Direction::North as usize) * 2;
        }
        list.add(from, to, Flag::DoublePush);
    }
}

impl Board {
    pub fn add_pawns_moves<const IS_WHITE: bool>(&self, list: &mut List) {
        let mut pushed: Bitboard;
        if IS_WHITE {
            pushed = self.0[Piece::WhitePawn as usize];
        } else {
            pushed = self.0[Piece::BlackPawn as usize];
        }
        pushed = pushed.shifted_forward::<IS_WHITE>();
        pushed &= !last_rank::<IS_WHITE>();
        pushed &= self.empty();
        add_single_pushes::<IS_WHITE>(pushed, list);

        pushed &= third_rank::<IS_WHITE>();
        pushed = pushed.shifted_forward::<IS_WHITE>();
        pushed &= self.empty();
        add_double_pushes::<IS_WHITE>(pushed, list);

        //TODO: captures, en passant, promotions, promotion captures
    }
}
