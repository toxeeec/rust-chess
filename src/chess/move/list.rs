use super::r#type::{Flag, Type};

#[derive(Debug)]
pub struct List(pub Vec<Type>);

impl List {
    pub fn new() -> Self {
        List(Vec::new())
    }
    pub fn add(&mut self, from: usize, to: usize, flag: Flag) {
        let move_type = Type::new(from, to, flag);

        self.0.push(move_type);
    }
}

#[cfg(test)]
mod tests {

    use crate::chess::Board;

    use super::*;

    #[test]
    fn list_starting_pos_pawns_white_test() {
        let starting_pos_pawns = "8/pppppppp/8/8/8/8/PPPPPPPP/8 w KQkq - 0 1";
        let board = Board::from_fen(starting_pos_pawns).unwrap();
        let mut list = List::new();
        board.add_pawns_moves::<false>(&mut list);
        assert_eq!(16, list.0.len());
    }
}
