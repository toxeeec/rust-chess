use super::fen;
use super::Bitboard;
use std::fmt;

#[derive(Clone, Copy)]
enum Piece {
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteKing,
    WhiteQueen,
    WhitePawn,

    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackKing,
    BlackQueen,
    BlackPawn,
}

const PIECE_ITEMS: [Piece; 12] = [
    Piece::WhiteRook,
    Piece::WhiteKnight,
    Piece::WhiteBishop,
    Piece::WhiteKing,
    Piece::WhiteQueen,
    Piece::WhitePawn,
    Piece::BlackRook,
    Piece::BlackKnight,
    Piece::BlackBishop,
    Piece::BlackKing,
    Piece::BlackQueen,
    Piece::BlackPawn,
];

pub const CHAR_PIECES: [char; 12] = ['R', 'N', 'B', 'K', 'Q', 'P', 'r', 'n', 'b', 'k', 'q', 'p'];

#[derive(Debug, PartialEq, Eq)]
pub struct Board(pub [Bitboard; 12]);

impl Board {
    pub fn new() -> Self {
        Self([Bitboard(0); 12])
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen(String::from(fen::STARTING_POS)).unwrap()
    }
}

const BOARD_STRING_LENGTH: usize = 19 * 9;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pieces = ['.'; 64];
        for piece in PIECE_ITEMS {
            let mut bb = self.0[piece as usize];
            while bb.0 > 0 {
                let i = bb.pop_lsb().unwrap();
                pieces[i] = CHAR_PIECES[piece as usize];
            }
        }

        let mut formatted = String::with_capacity(BOARD_STRING_LENGTH);
        for rank in (0..8).rev() {
            let mut rank_string = String::with_capacity(18);
            rank_string.push_str(format!("{}  ", rank + 1).as_str());
            for file in 0..8 {
                rank_string.push(pieces[rank * 8 + file]);
                if file < 7 {
                    rank_string.push(' ');
                } else {
                    rank_string.push('\n');
                }
            }
            formatted.push_str(rank_string.as_str());
        }
        formatted.push('\n');
        formatted.push_str("   A B C D E F G H");

        write!(f, "{}", formatted)
    }
}

#[cfg(test)]
mod tests {
    use crate::chess::Bitboard;

    use super::*;

    #[test]
    fn fen_starting_pos_test() {
        let board = Board::default();
        let expected = Board([
            Bitboard(0b10000001),
            Bitboard(0b01000010),
            Bitboard(0b00100100),
            Bitboard(0b00010000),
            Bitboard(0b00001000),
            Bitboard(0b11111111 << 8),
            Bitboard(0b10000001 << 56),
            Bitboard(0b01000010 << 56),
            Bitboard(0b00100100 << 56),
            Bitboard(0b00010000 << 56),
            Bitboard(0b00001000 << 56),
            Bitboard(0b11111111 << 48),
        ]);
        assert_eq!(board, expected);
    }
}
