use crate::chess::Board;

use super::r#type::{Flag, Type};

#[derive(Debug)]
pub struct List(pub Vec<Type>);

impl List {
    pub fn generate<const IS_WHITE: bool>(board: Board) -> Self {
        let mut list = Self(Vec::new());
        list.add_pawn_moves::<IS_WHITE>(board);
        list.add_knight_moves::<IS_WHITE>(board);

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

    const STARTING_POS_PAWNS: &str = "8/pppppppp/8/8/8/8/PPPPPPPP/8 w KQkq - 0 1";
    const KIWI_POS: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

    #[test]
    fn list_starting_pos_pawns_white_test() {
        let game = Game::from_fen(STARTING_POS_PAWNS).unwrap();
        assert_eq!(16, game.move_list.0.len());
    }

    #[test]
    fn list_starting_pos_pawns_black_test() {
        let game = Game::from_fen(STARTING_POS_PAWNS).unwrap();
        assert_eq!(16, game.move_list.0.len());
    }

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
}
