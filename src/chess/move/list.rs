use crate::chess::{game::Game, state::State, Board};

use super::r#type::{Flag, Type};

#[derive(Debug)]
pub struct List(pub Vec<Type>);

impl List {
    pub fn generate<const IS_WHITE: bool>(board: Board, state: State) -> Self {
        let mut list = Self(Vec::new());
        list.add_king_moves::<IS_WHITE>(board, state);
        list.add_pawn_moves::<IS_WHITE>(board);
        list.add_knight_moves::<IS_WHITE>(board);
        list.add_bishop_moves::<IS_WHITE>(board);
        list.add_rook_moves::<IS_WHITE>(board);
        list.add_queen_moves::<IS_WHITE>(board);
        //44

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
