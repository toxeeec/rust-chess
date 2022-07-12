#[derive(Debug)]
pub struct State {
    pub is_white: bool,
    pub has_ep_pawn: bool,
    pub can_castle_wl: bool,
    pub can_castle_wr: bool,
    pub can_castle_bl: bool,
    pub can_castle_br: bool,
}

impl State {
    pub const fn starting() -> Self {
        Self {
            is_white: true,
            has_ep_pawn: false,
            can_castle_wl: true,
            can_castle_wr: true,
            can_castle_bl: true,
            can_castle_br: true,
        }
    }
}
