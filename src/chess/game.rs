use super::{
    fen::{FenError, STARTING_POS},
    r#move::{Flag, List},
    state::State,
    Board,
};

#[derive(Debug, Clone, Copy)]
pub struct MoveCounter {
    pub half_clock: u32,
    pub full: u32,
}

pub struct Game {
    pub board: Board,
    pub state: State,
    pub ep_square: usize,
    pub move_list: List,
    pub move_counter: MoveCounter,
}

fn perft_inner(mut game: Game, nodes: &mut usize, depth: u32, captures: &mut usize) {
    if depth == 1 {
        *nodes += game.move_list.0.len();
        for m in &game.move_list.0 {
            if m.flag() == Flag::Capture {
                *captures += 1;
            }
        }
        return;
    }
    for (i, m) in game.move_list.0.iter().enumerate() {
        let mut ep_square = game.ep_square;
        let mut board = game.board;
        let mut state = game.state;
        if game.state.is_white {
            m.make::<true>(&mut board, &mut state, &mut ep_square);
        } else {
            m.make::<false>(&mut board, &mut state, &mut ep_square);
        }
        let move_list = if state.is_white {
            List::generate::<true>(board, state, ep_square)
        } else {
            List::generate::<false>(board, state, ep_square)
        };
        let move_counter = game.move_counter;
        let g = Game {
            board,
            state,
            ep_square,
            move_list,
            move_counter,
        };
        perft_inner(g, nodes, depth - 1, captures);
    }
}

impl Game {
    pub fn new() -> Self {
        Self::from_fen(STARTING_POS).unwrap()
    }

    pub fn perft(fen: &str, depth: u32) -> Result<usize, FenError> {
        let game = Game::from_fen(fen)?;
        let mut nodes = 0;
        let mut captures = 0;

        //TODO: add error handling for depth == 0

        perft_inner(game, &mut nodes, depth, &mut captures);

        Ok(nodes)
    }
}
