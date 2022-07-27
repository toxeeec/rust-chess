use crate::chess::{
    bitboard::{shift::Direction, RANK_1, RANK_3, RANK_6, RANK_8},
    board::Piece,
    state::State,
    Bitboard, Board,
};

use super::{list::List, magic::seen_squares_rook, masks::Pins, r#type::Flag};

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

fn add_promotions<const IS_WHITE: bool>(mut bb: Bitboard, list: &mut List) {
    while bb.0 > 0 {
        let to = bb.pop_lsb().unwrap();
        let from = if IS_WHITE {
            to - Direction::North as usize
        } else {
            to + Direction::North as usize
        };
        list.add(from, to, Flag::KnightPromotion);
        list.add(from, to, Flag::BishopPromotion);
        list.add(from, to, Flag::RookPromotion);
        list.add(from, to, Flag::QueenPromotion);
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

fn add_promotion_captures<const IS_WHITE: bool, const IS_LEFT: bool>(
    mut bb: Bitboard,
    list: &mut List,
) {
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
        list.add(from, to, Flag::KnightPromotionCapture);
        list.add(from, to, Flag::BishopPromotionCapture);
        list.add(from, to, Flag::RookPromotionCapture);
        list.add(from, to, Flag::QueenPromotionCapture);
    }
}

impl List {
    pub fn add_pawn_moves<const IS_WHITE: bool>(
        &mut self,
        board: Board,
        state: State,
        ep_square: usize,
        checkmask: Bitboard,
        pins: Pins,
    ) {
        let bb = if IS_WHITE {
            board.0[Piece::WhitePawn as usize]
        } else {
            board.0[Piece::BlackPawn as usize]
        };

        let not_diag_pinned = bb & !pins.diag;

        let mut pinned = (not_diag_pinned & pins.hv).shifted_forward::<IS_WHITE>() & pins.hv;

        let mut pushed = (not_diag_pinned & !pins.hv).shifted_forward::<IS_WHITE>() | pinned;
        pushed &= board.empty();
        let mut double_pushed = pushed;
        pushed &= checkmask;
        let mut promotions = pushed & last_rank::<IS_WHITE>();
        add_promotions::<IS_WHITE>(promotions, self);

        pushed &= !last_rank::<IS_WHITE>();
        add_single_pushes::<IS_WHITE>(pushed, self);

        double_pushed &= third_rank::<IS_WHITE>();
        double_pushed = double_pushed.shifted_forward::<IS_WHITE>();
        double_pushed &= board.empty();
        double_pushed &= checkmask;
        add_double_pushes::<IS_WHITE>(double_pushed, self);

        let not_hv_pinned = bb & !pins.hv;

        pinned = (not_hv_pinned & pins.diag).shifted_forward_left::<IS_WHITE>() & pins.diag;

        let mut shifted = (not_hv_pinned & !pins.diag).shifted_forward_left::<IS_WHITE>() | pinned;
        shifted &= board.enemy::<IS_WHITE>();
        shifted &= checkmask;
        promotions = shifted & last_rank::<IS_WHITE>();
        add_promotion_captures::<IS_WHITE, true>(promotions, self);

        shifted &= !last_rank::<IS_WHITE>();
        add_captures::<IS_WHITE, true>(shifted, self);

        pinned = (not_hv_pinned & pins.diag).shifted_forward_right::<IS_WHITE>() & pins.diag;
        shifted = (not_hv_pinned & !pins.diag).shifted_forward_right::<IS_WHITE>() | pinned;
        shifted &= board.enemy::<IS_WHITE>();
        shifted &= checkmask;

        promotions = shifted & last_rank::<IS_WHITE>();
        add_promotion_captures::<IS_WHITE, false>(promotions, self);

        shifted &= !last_rank::<IS_WHITE>();
        add_captures::<IS_WHITE, false>(shifted, self);

        if state.has_ep_pawn {
            let bb = if IS_WHITE {
                Bitboard::from_square(ep_square).shifted(Direction::SouthEast)
                    | Bitboard::from_square(ep_square).shifted(Direction::SouthWest)
            } else {
                Bitboard::from_square(ep_square).shifted(Direction::NorthEast)
                    | Bitboard::from_square(ep_square).shifted(Direction::NorthWest)
            } & not_hv_pinned;

            let ep_pawn = if IS_WHITE {
                ep_square - 8
            } else {
                ep_square + 8
            };

            let mut can_ep = bb;

            while can_ep.0 > 0 {
                let sq = can_ep.pop_lsb().unwrap();

                let mut queen_or_rook = if IS_WHITE {
                    board.0[Piece::BlackQueen as usize] | board.0[Piece::BlackRook as usize]
                } else {
                    board.0[Piece::WhiteQueen as usize] | board.0[Piece::WhiteRook as usize]
                };

                let king_bb = if IS_WHITE {
                    board.0[Piece::WhiteKing as usize]
                } else {
                    board.0[Piece::BlackKing as usize]
                };

                // https://lichess.org/editor/8/8/8/kq1pP1K1/8/8/8/8_w_-_d6_0_1
                while queen_or_rook.0 > 0 {
                    let qr_sq = queen_or_rook.pop_lsb().unwrap();
                    let occ = !board.empty() & !Bitboard::from_squares([sq, ep_pawn]);
                    if (seen_squares_rook(qr_sq, occ) & king_bb).0 > 0 {
                        can_ep &= !Bitboard::from_square(sq);
                        return;
                    }
                }

                let is_pinned = (Bitboard::from_square(sq) & pins.diag).0 > 0;
                let is_ep_square_pinned = (Bitboard::from_square(ep_square) & pins.diag).0 > 0;
                if !is_pinned || is_ep_square_pinned {
                    self.add(sq, ep_square, Flag::EnPassant);
                }
            }
        }
    }
}
