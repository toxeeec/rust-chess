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
    pub state: State,
    pub ep_square: usize,
    pub move_list: List,
    pub move_counter: MoveCounter,
}

impl Game {
    pub fn new() -> Self {
        Self::from_fen(STARTING_POS).unwrap()
    }
}
