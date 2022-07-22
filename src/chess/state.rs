#[derive(Debug, Clone, Copy)]
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

    pub const fn quiet(&self) -> Self {
        Self {
            is_white: !self.is_white,
            has_ep_pawn: false,
            can_castle_wl: self.can_castle_wl,
            can_castle_wr: self.can_castle_wr,
            can_castle_bl: self.can_castle_bl,
            can_castle_br: self.can_castle_br,
        }
    }

    pub const fn left_rook<const IS_WHITE: bool>(&self) -> Self {
        if IS_WHITE {
            Self {
                is_white: false,
                has_ep_pawn: false,
                can_castle_wl: false,
                can_castle_wr: self.can_castle_wr,
                can_castle_bl: self.can_castle_bl,
                can_castle_br: self.can_castle_br,
            }
        } else {
            Self {
                is_white: true,
                has_ep_pawn: false,
                can_castle_wl: self.can_castle_wl,
                can_castle_wr: self.can_castle_wr,
                can_castle_bl: false,
                can_castle_br: self.can_castle_br,
            }
        }
    }

    pub const fn right_rook<const IS_WHITE: bool>(&self) -> Self {
        if IS_WHITE {
            Self {
                is_white: false,
                has_ep_pawn: false,
                can_castle_wl: self.can_castle_wl,
                can_castle_wr: false,
                can_castle_bl: self.can_castle_bl,
                can_castle_br: self.can_castle_br,
            }
        } else {
            Self {
                is_white: true,
                has_ep_pawn: false,
                can_castle_wl: self.can_castle_wl,
                can_castle_wr: self.can_castle_wr,
                can_castle_bl: self.can_castle_bl,
                can_castle_br: false,
            }
        }
    }

    pub const fn double_push(&self) -> Self {
        Self {
            is_white: !self.is_white,
            has_ep_pawn: true,
            can_castle_wl: self.can_castle_wl,
            can_castle_wr: self.can_castle_wr,
            can_castle_bl: self.can_castle_bl,
            can_castle_br: self.can_castle_br,
        }
    }

    pub const fn king<const IS_WHITE: bool>(&self) -> Self {
        if IS_WHITE {
            Self {
                is_white: false,
                has_ep_pawn: false,
                can_castle_wl: false,
                can_castle_wr: false,
                can_castle_bl: self.can_castle_bl,
                can_castle_br: self.can_castle_br,
            }
        } else {
            Self {
                is_white: true,
                has_ep_pawn: false,
                can_castle_wl: self.can_castle_wl,
                can_castle_wr: self.can_castle_wr,
                can_castle_bl: false,
                can_castle_br: false,
            }
        }
    }
}
