use std::char::from_digit;

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

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SquareError {
    #[error("Expected length: 2, got: {0}")]
    Length(usize),
    #[error("Invalid file: {0}")]
    File(char),
    #[error("Invalid rank: {0}")]
    Rank(char),
}

pub fn name_to_square(name: &str) -> Result<usize, SquareError> {
    if name.len() != 2 {
        return Err(SquareError::Length(name.len()));
    }
    let name = name.to_lowercase();
    let mut name = name.bytes();
    let file_byte = name.next().unwrap();
    let file = file_byte.wrapping_sub(b'a');
    if file > 7 {
        return Err(SquareError::File(file_byte as char));
    }
    let rank_byte = name.next().unwrap();
    let rank = rank_byte - b'1';
    if rank > 7 {
        return Err(SquareError::Rank(rank_byte as char));
    }
    Ok((rank * 8 + file) as usize)
}

pub fn square_to_name(sq: usize) -> String {
    //TODO: error handling for > 63
    let file = sq % 8;
    let rank = sq / 8 + 1;
    let mut name = String::with_capacity(2);
    name.push((b'a' + file as u8) as char);
    name.push(from_digit(rank as u32, 10).unwrap());

    name
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
    #[case("Z1", Err(SquareError::File('z')))]
    #[case("a9", Err(SquareError::Rank('9')))]
    #[case("abc", Err(SquareError::Length(3)))]
    fn name_to_number_test(#[case] name: &str, #[case] expected: Result<usize, SquareError>) {
        assert_eq!(expected, name_to_square(name));
    }
}
