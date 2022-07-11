use super::board::CHAR_PIECES;
use super::square::name_to_number;
use super::state::State;
use super::Board;
use thiserror::Error;

pub const STARTING_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Error, Debug)]
pub enum FenError {
    #[error("Invalid FEN at field: {0}")]
    InvalidField(usize),
}

fn board(pieces: &str) -> Result<Board, FenError> {
    let mut board = Board::new();
    let mut sq = 56;
    for p in pieces.chars() {
        if let Some(i) = CHAR_PIECES.iter().position(|&x| x == p) {
            let bb = &mut board.0[i];
            bb.set(sq as usize);
            sq += 1;
        } else {
            match p {
                n @ '1'..='8' => sq += n.to_digit(10).unwrap(),
                '/' => sq -= 16,
                _ => return Err(FenError::InvalidField(1)),
            }
        }
    }
    Ok(board)
}

//TODO: ep square
fn state(side: &str, castling: &str, ep: &str) -> Result<State, FenError> {
    let is_white = match side {
        "w" => true,
        "b" => false,
        _ => return Err(FenError::InvalidField(2)),
    };

    let mut can_castle_wl = false;
    let mut can_castle_wr = false;
    let mut can_castle_bl = false;
    let mut can_castle_br = false;
    if castling == "-" {
        can_castle_wl = false;
        can_castle_wr = false;
        can_castle_bl = false;
        can_castle_br = false;
    } else {
        for letter in castling.chars() {
            match letter {
                'K' => can_castle_wl = true,
                'Q' => can_castle_wr = true,
                'k' => can_castle_bl = true,
                'q' => can_castle_br = true,
                _ => return Err(FenError::InvalidField(3)),
            }
        }
    };

    let has_ep_pawn = if ep == "-" {
        false
    } else {
        match name_to_number(ep) {
            Ok(_) => true,
            Err(_) => return Err(FenError::InvalidField(4)),
        }
    };

    Ok(State {
        is_white,
        has_ep_pawn,
        can_castle_wl,
        can_castle_wr,
        can_castle_bl,
        can_castle_br,
    })
}

//TODO: move counter
pub fn parse(fen: &str) -> Result<(Board, State), FenError> {
    let fields: [&str; 6] = match fen.split_whitespace().collect::<Vec<_>>().try_into() {
        Ok(fields) => fields,
        Err(_) => return Err(FenError::InvalidField(999)),
    };
    let b = board(fields[0])?;
    let s = state(fields[1], fields[2], fields[3])?;
    Ok((b, s))
}
