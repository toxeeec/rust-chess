use crate::chess::{
    bitboard::{shift::Direction, RANK_1, RANK_8},
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

fn add_single_pushes<const IS_WHITE: bool>(board: &Board, list: &mut List) {
    let mut pushed: Bitboard;
    if IS_WHITE {
        pushed = board.0[Piece::WhitePawn as usize];
    } else {
        pushed = board.0[Piece::BlackPawn as usize];
    }
    pushed |= last_rank::<IS_WHITE>();

    while pushed.0 > 0 {
        let from = pushed.pop_lsb().unwrap();
        let to: usize;
        if IS_WHITE {
            to = from - Direction::North as usize;
        } else {
            to = from + Direction::North as usize;
        }
        list.add(from, to, Flag::Quiet);
    }
}

impl Board {
    pub fn add_pawns_moves<const IS_WHITE: bool>(&self, list: &mut List) {
        add_single_pushes::<IS_WHITE>(self, list);

        //TODO: double_pushes, captures, en passant, promotions, promotion captures
    }
}
