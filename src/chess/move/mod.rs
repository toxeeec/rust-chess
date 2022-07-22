mod figures;
mod list;
mod lookup;
mod magic;
mod masks;
mod pawns;
mod r#type;

pub use list::List;
pub use lookup::CHECK_PATH;
pub use r#type::{Flag, Type};

use self::figures::{is_left_rook, is_right_rook};

use super::{board::Piece, game::Game, state::State, Bitboard, Board};

impl Type {
    pub fn make<const IS_WHITE: bool>(
        &self,
        board: &mut Board,
        state: &mut State,
        ep_square: &mut usize,
    ) {
        let from = self.from();
        let to = self.to();
        let mut flag = self.flag();
        let from_bb = Bitboard::from_square(from);
        let to_bb = Bitboard::from_square(to);
        let piece_type = match flag {
            Flag::Quiet | Flag::Capture => {
                let mut piece = Piece::WhiteRook;
                for (i, bb) in board.0.iter().enumerate() {
                    if (from_bb & *bb).0 > 0 {
                        piece = i.into();
                        break;
                    }
                }

                piece
            }
            Flag::KingCastle | Flag::QueenCastle => {
                if IS_WHITE {
                    Piece::WhiteKing
                } else {
                    Piece::BlackKing
                }
            }
            _ => {
                if IS_WHITE {
                    Piece::WhitePawn
                } else {
                    Piece::BlackPawn
                }
            }
        };
        match flag {
            Flag::Quiet | Flag::Capture => {
                if flag == Flag::Capture {
                    let mut captured_type = Piece::WhiteRook;
                    for (i, bb) in board.0.iter().enumerate() {
                        if (to_bb & *bb).0 > 0 {
                            captured_type = i.into();
                            break;
                        }
                    }
                    let captured_bb = &mut board.0[captured_type as usize];
                    *captured_bb &= !to_bb;
                }
                let bb = &mut board.0[piece_type as usize];
                *bb &= !from_bb;
                *bb |= to_bb;

                match piece_type {
                    Piece::WhiteRook | Piece::BlackRook => {
                        if is_left_rook::<IS_WHITE>(from_bb) {
                            *state = state.left_rook::<IS_WHITE>();
                        } else if is_right_rook::<IS_WHITE>(from_bb) {
                            *state = state.right_rook::<IS_WHITE>();
                        } else {
                            *state = state.quiet();
                        }
                    }
                    Piece::WhiteKing | Piece::BlackKing => *state = state.king::<IS_WHITE>(),
                    _ => *state = state.quiet(),
                }
            }
            Flag::KingCastle => {
                let king_bb = &mut board.0[piece_type as usize];
                if IS_WHITE {
                    *king_bb &= !Bitboard::from_square(4);
                    *king_bb |= Bitboard::from_square(6);
                } else {
                    *king_bb &= !Bitboard::from_square(60);
                    *king_bb |= Bitboard::from_square(62);
                }
                let rook_bb = if IS_WHITE {
                    &mut board.0[Piece::WhiteRook as usize]
                } else {
                    &mut board.0[Piece::BlackRook as usize]
                };
                if IS_WHITE {
                    *rook_bb &= !Bitboard::from_square(7);
                    *rook_bb |= Bitboard::from_square(5);
                } else {
                    *rook_bb &= !Bitboard::from_square(63);
                    *rook_bb |= Bitboard::from_square(61);
                }

                *state = state.king::<IS_WHITE>();
            }
            Flag::QueenCastle => {
                let king_bb = &mut board.0[piece_type as usize];
                if IS_WHITE {
                    *king_bb &= !Bitboard::from_square(4);
                    *king_bb |= Bitboard::from_square(2);
                } else {
                    *king_bb &= !Bitboard::from_square(60);
                    *king_bb |= Bitboard::from_square(58);
                }
                let rook_bb = if IS_WHITE {
                    &mut board.0[Piece::WhiteRook as usize]
                } else {
                    &mut board.0[Piece::BlackRook as usize]
                };
                if IS_WHITE {
                    *rook_bb &= !Bitboard::from_square(0);
                    *rook_bb |= Bitboard::from_square(3);
                } else {
                    *rook_bb &= !Bitboard::from_square(56);
                    *rook_bb |= Bitboard::from_square(59);
                }

                *state = state.king::<IS_WHITE>();
            }
            Flag::DoublePush => {
                let bb = &mut board.0[piece_type as usize];

                *bb &= !from_bb;
                *bb |= to_bb;

                *ep_square = if IS_WHITE { to - 8 } else { to + 8 };
                *state = state.double_push();
            }

            Flag::EnPassant => {
                let bb = &mut board.0[piece_type as usize];

                *bb &= !from_bb;
                *bb |= to_bb;

                let captured_bb = if IS_WHITE {
                    &mut board.0[Piece::BlackPawn as usize]
                } else {
                    &mut board.0[Piece::WhitePawn as usize]
                };

                if IS_WHITE {
                    *captured_bb &= !Bitboard::from_square(to - 8);
                } else {
                    *captured_bb &= !Bitboard::from_square(to + 8);
                }

                *state = state.quiet();
            }
            _ => {
                let bb = &mut board.0[piece_type as usize];
                *bb &= !from_bb;

                if flag as u32 & Flag::Capture as u32 > 0 {
                    let mut captured_type = Piece::WhiteRook;
                    for (i, bb) in board.0.iter().enumerate() {
                        if (to_bb & *bb).0 > 0 {
                            captured_type = i.into();
                            break;
                        }
                    }
                    let captured_bb = &mut board.0[captured_type as usize];
                    *captured_bb &= !to_bb;
                }

                flag = (flag as u32 & 0b1011).into();

                let promoted_bb = match flag {
                    Flag::KnightPromotion => {
                        if IS_WHITE {
                            &mut board.0[Piece::WhiteKnight as usize]
                        } else {
                            &mut board.0[Piece::BlackKnight as usize]
                        }
                    }
                    Flag::BishopPromotion => {
                        if IS_WHITE {
                            &mut board.0[Piece::WhiteBishop as usize]
                        } else {
                            &mut board.0[Piece::BlackBishop as usize]
                        }
                    }
                    Flag::RookPromotion => {
                        if IS_WHITE {
                            &mut board.0[Piece::WhiteRook as usize]
                        } else {
                            &mut board.0[Piece::BlackRook as usize]
                        }
                    }
                    _ => {
                        if IS_WHITE {
                            &mut board.0[Piece::WhiteQueen as usize]
                        } else {
                            &mut board.0[Piece::BlackQueen as usize]
                        }
                    }
                };
                *promoted_bb |= to_bb;

                *state = state.quiet();
            }
        }
    }
}
