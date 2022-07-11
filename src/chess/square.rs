use thiserror::Error;

pub const fn max(x: usize, y: usize) -> usize {
    if x > y {
        x
    } else {
        y
    }
}

pub const fn min(x: usize, y: usize) -> usize {
    if x < y {
        x
    } else {
        y
    }
}

const fn squares_distances() -> [[usize; 64]; 64] {
    let mut distances = [[0; 64]; 64];
    let mut sq1 = 0;
    while sq1 < 64 {
        let mut sq2 = 0;
        while sq2 < 64 {
            let (sq1_rank, sq2_rank) = (sq1 / 8, sq2 / 8);
            let (sq1_file, sq2_file) = (sq1 % 8, sq2 % 8);
            distances[sq1][sq2] = max(sq1_rank.abs_diff(sq2_rank), sq1_file.abs_diff(sq2_file));
            sq2 += 1;
        }
        sq1 += 1;
    }
    distances
}

#[derive(Error, Debug, PartialEq)]
pub enum SquareError {
    #[error("Length of square name must be 2, but given {0}")]
    InvalidLength(usize),
    #[error("Invalid file: {0}")]
    InvalidFile(char),
    #[error("Invalid rank: {0}")]
    InvalidRank(char),
}

pub fn name_to_number(name: &str) -> Result<usize, SquareError> {
    if name.len() != 2 {
        return Err(SquareError::InvalidLength(name.len()));
    }
    let name = name.to_lowercase();
    let mut name = name.bytes();
    let file_byte = name.next().unwrap();
    let file = file_byte.wrapping_sub(b'a');
    if file > 7 {
        return Err(SquareError::InvalidFile(file_byte as char));
    }
    let rank_byte = name.next().unwrap();
    let rank = rank_byte - b'1';
    if rank > 7 {
        return Err(SquareError::InvalidRank(rank_byte as char));
    }
    Ok((rank * 8 + file) as usize)
}

pub const SQUARES_DISTANCES: [[usize; 64]; 64] = squares_distances();

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0, 0)]
    #[case(63, 0, 7)]
    #[case(0, 33, 4)]
    #[case(12, 0, 4)]
    fn squares_distances_test(#[case] sq1: usize, #[case] sq2: usize, #[case] expected: usize) {
        assert_eq!(expected, SQUARES_DISTANCES[sq1][sq2]);
    }

    #[rstest]
    #[case("A1", Ok(0))]
    #[case("h8", Ok(63))]
    #[case("Z1", Err(SquareError::InvalidFile('z')))]
    #[case("a9", Err(SquareError::InvalidRank('9')))]
    #[case("abc", Err(SquareError::InvalidLength(3)))]
    fn name_to_number_test(#[case] name: &str, #[case] expected: Result<usize, SquareError>) {
        assert_eq!(expected, name_to_number(name));
    }
}
