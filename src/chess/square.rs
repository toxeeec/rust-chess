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
}
