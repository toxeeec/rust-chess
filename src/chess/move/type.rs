/*
    bits range - meaning
    15-10 - from square
    9-4 - to square
    3 - promotion
    2 - capture
    1 - special 1
    0 - special 0
*/

#[repr(u16)]
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
}

#[derive(Debug, PartialEq)]
pub struct Type(pub u16);

impl Type {
    pub const fn new(from: u16, to: u16, flag: u16) -> Self {
        let mut move_type = from << 10;
        move_type |= to << 4;
        move_type |= flag;

        Self(move_type)
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
        let flag = Flag::DoublePush as u16;
        assert_eq!(expected, Type::new(from, to, flag));
    }
}
