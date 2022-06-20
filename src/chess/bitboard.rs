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
    pub fn set(&mut self, i: usize) {
        assert!(i < 64);
        self.0 |= 1 << i;
    }

    pub fn get(self, i: usize) -> u64 {
        assert!(i < 64);
        self.0 >> i & 1
    }

    pub fn clear(&mut self, i: usize) {
        assert!(i < 64);
        self.0 &= !(1 << i);
    }

    pub fn get_lsb(mut self) -> Option<usize> {
        let mut i = 0;
        for _ in 0..64 {
            if self.0 & 1 == 1 {
                return Some(i);
            } else {
                self.0 >>= 1;
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
    #[case(1<<63, 63, 1)]
    #[case(0b11001100, 6, 1)]
    #[should_panic]
    #[case(0, 64, 0)]
    fn get_test(#[case] bb: u64, #[case] i: usize, #[case] expected: u64) {
        let bb = Bitboard(bb);
        assert_eq!(expected, bb.get(i));
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
