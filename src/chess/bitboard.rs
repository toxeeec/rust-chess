use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Bitboard(pub u64);

const BITBOARD_STRING_LENGTH: usize = 16 * 8 - 1;

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:064b}", self.0)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b: Vec<char> = format!("{:064b}", self.0).chars().rev().collect();
        let mut formatted = String::with_capacity(BITBOARD_STRING_LENGTH);
        for rank in (0..8).rev() {
            for file in 0..8 {
                formatted.push(b[rank * 8 + file] as char);
                if file < 7 {
                    formatted.push(' ');
                } else if rank > 0 {
                    formatted.push('\n');
                }
            }
        }
        write!(f, "{}", formatted)
    }
}

impl Bitboard {
    pub fn set(&mut self, i: u32) {
        assert!(i < 64);
        self.0 |= 1 << i;
    }

    //TODO: get_lsb
    //returns index of lsb
    pub fn pop_lsb(&mut self) -> Option<usize> {
        let (mut i, mut n) = (0, self.0);
        for _ in 0..64 {
            if n & 1 == 1 {
                self.0 &= !(1 << i);
                return Some(i);
            } else {
                n >>= 1;
                i += 1;
            }
        }
        None
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
    fn set_test(#[case] bb: u64, #[case] i: u32, #[case] expected: u64) {
        let mut bb = Bitboard(bb);
        bb.set(i);
        assert_eq!(expected, bb.0);
    }

    #[rstest]
    #[case(1, Some(0))]
    #[case(1 << 63, Some(63))]
    #[case(0b10100000, Some(5))]
    #[case(0, None)]
    fn pop_lsb_test(#[case] bb: u64, #[case] expected: Option<usize>) {
        let mut bb = Bitboard(bb);
        assert_eq!(expected, bb.pop_lsb());
    }
}
