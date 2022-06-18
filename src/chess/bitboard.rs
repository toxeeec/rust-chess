use std::fmt;
pub struct Bitboard(pub u64);

const BITBOARD_STRING_LENGTH: usize = 16 * 8 - 1;

impl fmt::Debug for Bitboard {
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
        self.0 |= 1 << i;
        assert!(i < 64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0, 1)]
    #[case(0, 63, 2_u64.pow(63))]
    #[case(0b10111111, 6, u8::MAX.into())]
    fn set_test(#[case] bb: u64, #[case] i: u32, #[case] expected: u64) {
        let mut bb = Bitboard(bb);
        bb.set(i);
        assert_eq!(expected, bb.0);
    }
}
