use super::fen;
use super::Bitboard;
use super::Fen;

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    white_rook: Bitboard,
    white_knight: Bitboard,
    white_bishop: Bitboard,
    white_king: Bitboard,
    white_queen: Bitboard,
    white_pawn: Bitboard,

    black_rook: Bitboard,
    black_knight: Bitboard,
    black_bishop: Bitboard,
    black_king: Bitboard,
    black_queen: Bitboard,
    black_pawn: Bitboard,
}

impl Board {
    pub fn bitboard_from_char(&mut self, piece: char) -> Option<&mut Bitboard> {
        match piece {
            'R' => Some(&mut self.white_rook),
            'N' => Some(&mut self.white_knight),
            'B' => Some(&mut self.white_bishop),
            'K' => Some(&mut self.white_king),
            'Q' => Some(&mut self.white_queen),
            'P' => Some(&mut self.white_pawn),
            'r' => Some(&mut self.black_rook),
            'n' => Some(&mut self.black_knight),
            'b' => Some(&mut self.black_bishop),
            'k' => Some(&mut self.black_king),
            'q' => Some(&mut self.black_queen),
            'p' => Some(&mut self.black_pawn),
            _ => None,
        }
    }

    pub fn empty() -> Self {
        Self {
            white_rook: Bitboard(0),
            white_knight: Bitboard(0),
            white_bishop: Bitboard(0),
            white_king: Bitboard(0),
            white_queen: Bitboard(0),
            white_pawn: Bitboard(0),

            black_rook: Bitboard(0),
            black_knight: Bitboard(0),
            black_bishop: Bitboard(0),
            black_king: Bitboard(0),
            black_queen: Bitboard(0),
            black_pawn: Bitboard(0),
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Fen::new(String::from(fen::STARTING_POS)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::chess::Bitboard;

    use super::*;

    #[test]
    fn fen_starting_pos_test() {
        let board = Board::default();
        let expected = Board {
            white_rook: Bitboard(0b10000001),
            white_knight: Bitboard(0b01000010),
            white_bishop: Bitboard(0b00100100),
            white_king: Bitboard(0b00010000),
            white_queen: Bitboard(0b00001000),
            white_pawn: Bitboard(0b11111111 << 8),

            black_rook: Bitboard(0b10000001 << 56),
            black_knight: Bitboard(0b01000010 << 56),
            black_bishop: Bitboard(0b00100100 << 56),
            black_king: Bitboard(0b00010000 << 56),
            black_queen: Bitboard(0b00001000 << 56),
            black_pawn: Bitboard(0b11111111 << 48),
        };
        assert_eq!(board, expected);
    }
}
