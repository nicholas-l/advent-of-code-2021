use std::io::BufRead;

use itertools::Itertools;

pub fn star_one(input: impl BufRead) -> usize {
    input
        .lines()
        .map(|x| x.unwrap().parse::<usize>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

pub fn star_two(input: impl BufRead) -> usize {
    input
        .lines()
        .map(|x| x.unwrap().parse::<usize>().unwrap())
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"199
200
208
210
200
207
240
269
260
263"
            )),
            7
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"199
200
208
210
200
207
240
269
260
263"
            )),
            5
        );
    }
}
