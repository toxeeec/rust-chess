use super::{
    fen::{self, FenError, STARTING_POS},
    r#move::List,
    state::State,
    Board,
};

pub struct Game {
    pub board: Board,
    pub move_list: List,
    pub state: State,
}

impl Game {
    pub fn new() -> Self {
        Self::from_fen(STARTING_POS).unwrap()
    }

    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        //TODO: add error handling
        let (board, state) = match fen::parse(fen) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        let move_list = if state.is_white {
            List::generate::<true>(board)
        } else {
            List::generate::<false>(board)
        };
        Ok(Self {
            board,
            move_list,
            state,
        })
    }
}
