use super::board::CHAR_PIECES;
use super::Board;

pub const STARTING_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Fen(pub String);

impl Fen {
    pub fn new(fen: String) -> Result<Board, String> {
        let fen: Vec<&str> = fen.split(" ").collect();
        let mut board = Board::empty();
        let mut square = 56;
        for p in fen[0].chars() {
            if let Some(i) = CHAR_PIECES.iter().position(|&x| x == p) {
                let bb = &mut board.0[i];
                bb.set(square);
                square += 1;
            } else {
                match p {
                    n @ '1'..='8' => square += n.to_digit(10).unwrap(),
                    '/' => square -= 16,
                    _ => return Err(String::from("Invalid fen")),
                }
            }
        }
        Ok(board)
    }
}
