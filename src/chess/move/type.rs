/*
    bits range - meaning
    15-10 - from square
    9-4 - to square
    3 - promotion
    2 - capture
    1 - special 1
    0 - special 0
*/

use std::fmt;

use crate::chess::square::square_to_name;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Flag {
    Quiet = 0b0,
    DoublePush = 0b1,
    KingCastle = 0b10,
    QueenCastle = 0b11,
    Capture = 0b100,
    EnPassant = 0b101,
    KnightPromotion = 0b1000,
    BishopPromotion = 0b1001,
    RookPromotion = 0b1010,
    QueenPromotion = 0b1011,
    KnightPromotionCapture = 0b1100,
    BishopPromotionCapture = 0b1101,
    RookPromotionCapture = 0b1110,
    QueenPromotionCapture = 0b1111,
}

impl From<u32> for Flag {
    fn from(flag: u32) -> Self {
        match flag {
            0b0 => Flag::Quiet,
            0b1 => Flag::DoublePush,
            0b10 => Flag::KingCastle,
            0b11 => Flag::QueenCastle,
            0b100 => Flag::Capture,
            0b101 => Flag::EnPassant,
            0b1000 => Flag::KnightPromotion,
            0b1001 => Flag::BishopPromotion,
            0b1010 => Flag::RookPromotion,
            0b1011 => Flag::QueenPromotion,
            0b1100 => Flag::KnightPromotionCapture,
            0b1101 => Flag::BishopPromotionCapture,
            0b1110 => Flag::RookPromotionCapture,
            0b1111 => Flag::QueenPromotionCapture,
            _ => panic!("Unknown flag: {:?}", flag),
        }
    }
}

//TODO: contain piece type
#[derive(PartialEq, Eq)]
pub struct Type(pub u32);

impl Type {
    pub const fn new(from: usize, to: usize, flag: Flag) -> Self {
        let mut move_type = from << 10;
        move_type |= to << 4;
        move_type |= flag as usize;

        Self(move_type as u32)
    }
    pub const fn from(&self) -> usize {
        (self.0 >> 10) as usize
    }
    pub const fn to(&self) -> usize {
        (self.0 >> 4 & 0b111111) as usize
    }
    pub fn flag(&self) -> Flag {
        let flag: Flag = (self.0 & 0b1111).into();
        flag
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(from: {}, to: {}, flag: {:?})",
            square_to_name(self.from()),
            square_to_name(self.to()),
            self.flag()
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_test() {
        let expected = Type(0b0011000111000001);
        let from = 12; // e2
        let to = 28; // e4
        let flag = Flag::DoublePush;
        assert_eq!(expected, Type::new(from, to, flag));
    }
}
