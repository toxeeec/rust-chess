use crate::chess::{board::Piece, Board};

use super::{lookup::KNIGHT, r#type::Flag, List};

impl List {
    pub fn add_knight_moves<const IS_WHITE: bool>(&mut self, board: Board) {
        let mut bb = if IS_WHITE {
            board.0[Piece::WhiteKnight as usize]
        } else {
            board.0[Piece::BlackKnight as usize]
        };

        while bb.0 > 0 {
            let from = bb.pop_lsb().unwrap();
            let mut moves = KNIGHT[from] & board.empty();
            while moves.0 > 0 {
                let to = moves.pop_lsb().unwrap();
                self.add(from, to, Flag::Quiet);
            }

            let mut captures = KNIGHT[from] & board.enemy::<IS_WHITE>();
            while captures.0 > 0 {
                let to = captures.pop_lsb().unwrap();
                self.add(from, to, Flag::Capture);
            }
        }
        //TODO: checks
    }
}
