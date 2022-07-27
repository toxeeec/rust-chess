use crate::chess::{board::Piece, state::State, Bitboard, Board};

use super::{
    lookup::KING,
    masks::{checkmask, pinmask},
    r#type::{Flag, Type},
};

#[derive(Debug)]
pub struct List(pub Vec<Type>);

impl List {
    pub fn generate<const IS_WHITE: bool>(board: Board, state: State, ep_square: usize) -> Self {
        let mut list = Self(Vec::new());
        let king_sq = if IS_WHITE {
            board.0[Piece::BlackKing as usize]
        } else {
            board.0[Piece::WhiteKing as usize]
        }
        .pop_lsb()
        .unwrap();

        let mut banned = KING[king_sq];

        let pins = pinmask::<IS_WHITE>(board);
        let checkmask = checkmask::<IS_WHITE>(board, &mut banned);
        list.add_king_moves::<IS_WHITE>(board, state, banned);
        if checkmask == Bitboard(0) {
            return list;
        }
        list.add_pawn_moves::<IS_WHITE>(board, state, ep_square, checkmask, pins);
        list.add_knight_moves::<IS_WHITE>(board, checkmask, pins);
        list.add_bishop_moves::<IS_WHITE>(board, checkmask, pins);
        list.add_rook_moves::<IS_WHITE>(board, checkmask, pins);
        list.add_queen_moves::<IS_WHITE>(board, checkmask, pins);

        list
    }
    pub fn add(&mut self, from: usize, to: usize, flag: Flag) {
        let move_type = Type::new(from, to, flag);

        self.0.push(move_type);
    }
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    const KIWI_POS: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    const CHECK_POS: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    const PIN_POS: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    const LEGAL_EP_POS: &str = "kq6/8/8/3pP3/8/6K1/8/8 w - d6 0 1";
    const ILLEGAL_EP_POS: &str = "8/8/8/kq1pP1K1/8/8/8/8 w - d6 0 1";
    const PROMOTION_POS: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    const ILLEGAL_CASTLE_POS: &str =
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPB1PPP/R3K2R w KQkq - 0 1";

    #[rstest]
    #[case(crate::chess::fen::STARTING_POS, 4, 197281)]
    #[case(KIWI_POS, 5, 193690690)]
    #[case(CHECK_POS, 5, 15833292)]
    #[case(PIN_POS, 5, 674624)]
    #[case(LEGAL_EP_POS, 1, 9)]
    #[case(ILLEGAL_EP_POS, 1, 9)]
    #[case(ILLEGAL_CASTLE_POS, 1, 43)]
    #[case(PROMOTION_POS, 5, 89941194)]
    fn perft_test(#[case] fen: &str, #[case] depth: u32, #[case] expected: usize) {
        let nodes = crate::chess::Game::perft(fen, depth).unwrap();
        assert_eq!(expected, nodes);
    }
}
