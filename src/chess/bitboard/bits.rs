use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, ShlAssign, Shr, ShrAssign};

use super::Bitboard;

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitOr<u64> for Bitboard {
    type Output = u64;
    fn bitor(self, rhs: u64) -> Self::Output {
        self.0 | rhs
    }
}

impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitAnd<u64> for Bitboard {
    type Output = u64;
    fn bitand(self, rhs: u64) -> Self::Output {
        self.0 & rhs
    }
}

impl BitAndAssign<u64> for Bitboard {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 &= rhs;
    }
}

impl Shl<usize> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl ShlAssign<usize> for Bitboard {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}

impl Shr<usize> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl ShrAssign<usize> for Bitboard {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Bitboard {
    pub fn set(&mut self, i: usize) {
        assert!(i < 64);
        *self |= 1 << i;
    }

    pub fn clear(&mut self, i: usize) {
        assert!(i < 64);
        *self &= !(1 << i);
    }

    pub fn get_lsb(mut self) -> Option<usize> {
        let mut i = 0;
        for _ in 0..64 {
            if self & 1 == 1 {
                return Some(i);
            } else {
                self >>= 1;
                i += 1;
            }
        }
        None
    }

    pub fn pop_lsb(&mut self) -> Option<usize> {
        let lsb = self.get_lsb();
        match lsb {
            Some(lsb) => {
                self.clear(lsb);
                Some(lsb)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0, 1)]
    #[case(0, 63, 1<<63)]
    #[case(0b10111111, 6, u8::MAX.into())]
    #[should_panic]
    #[case(0, 64, 0)]
    fn set_test(#[case] bb: u64, #[case] i: usize, #[case] expected: u64) {
        let mut bb = Bitboard(bb);
        bb.set(i);
        assert_eq!(expected, bb.0);
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(1<<63, 63, 0)]
    #[case(0b11001100, 6, 0b10001100)]
    #[should_panic]
    #[case(0, 64, 0)]
    fn clear_test(#[case] bb: u64, #[case] i: usize, #[case] expected: u64) {
        let mut bb = Bitboard(bb);
        bb.clear(i);
        assert_eq!(expected, bb.0);
    }
    #[rstest]
    #[case(1, Some(0))]
    #[case(1 << 63, Some(63))]
    #[case(0b10100000, Some(5))]
    #[case(0, None)]
    fn get_lsb_test(#[case] bb: u64, #[case] expected: Option<usize>) {
        let bb = Bitboard(bb);
        assert_eq!(expected, bb.get_lsb());
    }

    #[rstest]
    #[case(1, 0)]
    #[case(1 << 63, 0)]
    #[case(0b10100000, 0b10000000)]
    #[case(0, 0)]
    fn pop_lsb_test(#[case] bb: u64, #[case] expected: u64) {
        let mut bb = Bitboard(bb);
        bb.pop_lsb();
        assert_eq!(expected, bb.0);
    }
}
