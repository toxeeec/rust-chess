use crate::chess::{state::State, Bitboard, Board};

use super::{
    masks::checkmask,
    r#type::{Flag, Type},
};

#[derive(Debug)]
pub struct List(pub Vec<Type>);

impl List {
    pub fn generate<const IS_WHITE: bool>(board: Board, state: State) -> Self {
        let mut list = Self(Vec::new());
        let mut banned = Bitboard(0);
        let checkmask = checkmask::<IS_WHITE>(board, &mut banned);
        list.add_king_moves::<IS_WHITE>(board, state, banned);
        list.add_pawn_moves::<IS_WHITE>(board, checkmask);
        list.add_knight_moves::<IS_WHITE>(board, checkmask);
        list.add_bishop_moves::<IS_WHITE>(board, checkmask);
        list.add_rook_moves::<IS_WHITE>(board, checkmask);
        list.add_queen_moves::<IS_WHITE>(board, checkmask);

        list
    }
    pub fn add(&mut self, from: usize, to: usize, flag: Flag) {
        let move_type = Type::new(from, to, flag);

        self.0.push(move_type);
    }
}

#[cfg(test)]
mod tests {

    use crate::chess::game::Game;

    const KIWI_POS: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    const CHECK_POS: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P1KPP/R2QR3 w kq - 0 1";
    const PIN_POS: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";

    #[test]
    fn list_starting_pos_test() {
        let game = Game::new();
        assert_eq!(20, game.move_list.0.len());
    }

    #[test]
    fn list_kiwi_pos_test() {
        let game = Game::from_fen(KIWI_POS).unwrap();
        assert_eq!(48, game.move_list.0.len());
    }

    #[test]
    fn list_check_pos_test() {
        let game = Game::from_fen(CHECK_POS).unwrap();
        assert_eq!(8, game.move_list.0.len());
    }

    #[test]
    fn list_pin_pos_test() {
        let game = Game::from_fen(PIN_POS).unwrap();
        assert_eq!(14, game.move_list.0.len());
    }
}
