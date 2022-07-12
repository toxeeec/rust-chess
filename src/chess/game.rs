use super::{
    fen::{self, FenError, STARTING_POS},
    r#move::List,
    state::State,
    Board,
};

#[derive(Debug)]
pub struct MoveCounter {
    pub half_clock: usize,
    pub full: usize,
}

pub struct Game {
    pub board: Board,
    pub move_list: List,
    pub state: State,
    pub move_counter: MoveCounter,
}

impl Game {
    pub fn new() -> Self {
        Self::from_fen(STARTING_POS).unwrap()
    }

    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        let (board, state, move_counter) = match fen::parse(fen) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        let move_list = if state.is_white {
            List::generate::<true>(board, state)
        } else {
            List::generate::<false>(board, state)
        };
        Ok(Self {
            board,
            move_list,
            state,
            move_counter,
        })
    }
}
