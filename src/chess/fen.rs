use super::board::CHAR_PIECES;
use super::game::MoveCounter;
use super::square::name_to_number;
use super::state::State;
use super::Board;
use thiserror::Error;

pub const STARTING_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Error, Debug)]
pub enum FenError {
    #[error("Invalid FEN at field: {0}")]
    InvalidField(usize),
    #[error("FEN must contain 6 fields")]
    InvalidLength,
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

fn moves(half: &str, full: &str) -> Result<MoveCounter, FenError> {
    let half_clock = match half.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err(FenError::InvalidField(5)),
    };

    let full = match full.parse::<usize>() {
        n @ Ok(1..) => n.unwrap(),
        _ => return Err(FenError::InvalidField(6)),
    };

    Ok(MoveCounter { half_clock, full })
}

pub fn parse(fen: &str) -> Result<(Board, State, MoveCounter), FenError> {
    let fields: [&str; 6] = match fen.split_whitespace().collect::<Vec<_>>().try_into() {
        Ok(fields) => fields,
        Err(_) => return Err(FenError::InvalidLength),
    };
    let b = board(fields[0])?;
    let s = state(fields[1], fields[2], fields[3])?;
    let m = moves(fields[4], fields[5])?;
    Ok((b, s, m))
}

mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(STARTING_POS, true)]
    #[case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", false)]
    fn parse_test(#[case] fen: &str, #[case] is_ok: bool) {
        assert_eq!(is_ok, parse(fen).is_ok());
    }

    #[rstest]
    #[case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", true)]
    #[case("AAAAAAAA/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", false)]
    fn board_test(#[case] field: &str, #[case] is_ok: bool) {
        assert_eq!(is_ok, board(field).is_ok());
    }

    #[rstest]
    #[case("w", "KQkq", "-", true)]
    #[case("a", "KQkq", "-", false)]
    #[case("w", "AQkq", "-", false)]
    #[case("w", "KQkq", "A", false)]
    fn state_test(
        #[case] side: &str,
        #[case] castling: &str,
        #[case] ep: &str,
        #[case] is_ok: bool,
    ) {
        assert_eq!(is_ok, state(side, castling, ep).is_ok());
    }

    #[rstest]
    #[case("0", "1", true)]
    #[case("0", "0", false)]
    #[case("A", "1", false)]
    #[case("0", "a", false)]
    fn moves_test(#[case] half: &str, #[case] full: &str, #[case] is_ok: bool) {
        assert_eq!(is_ok, moves(half, full).is_ok());
    }
}
