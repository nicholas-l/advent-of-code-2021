use std::{collections::HashMap, io::BufRead};

type Point = (usize, usize);

fn parse_input(input: impl BufRead) -> impl Iterator<Item = (Point, Point)> {
    input.lines().map(|l| {
        let l = l.unwrap();
        let (s, e) = l.split_once("->").unwrap();
        let start = {
            let (x, y) = s.split_once(",").unwrap();
            (
                x.trim().parse::<usize>().unwrap(),
                y.trim()
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("{}", y)),
            )
        };
        let end = {
            let (x, y) = e.split_once(",").unwrap();
            (
                x.trim().parse::<usize>().unwrap(),
                y.trim().parse::<usize>().unwrap(),
            )
        };
        if start.0 <= end.0 {
            (start, end)
        } else {
            (end, start)
        }
    })
}

pub fn star_one(input: impl BufRead) -> usize {
    let freq = parse_input(input)
        .filter(|(start, end)| start.0 == end.0 || start.1 == end.1)
        .fold(HashMap::new(), |mut freq, (start, end)| {
            // Vertical
            if start.0 == end.0 {
                for i in start.1.min(end.1)..=start.1.max(end.1) {
                    let key = (start.0, i);
                    *freq.entry(key).or_insert(0) += 1;
                }
            }
            // Horizontal
            if start.1 == end.1 {
                for i in start.0.min(end.0)..=start.0.max(end.0) {
                    let key = (i, start.1);
                    *freq.entry(key).or_insert(0) += 1;
                }
            }
            freq
        });

    freq.into_iter().filter(|(_key, value)| value > &1).count()
}

pub fn star_two(input: impl BufRead) -> usize {
    let freq = parse_input(input).fold(HashMap::new(), |mut freq, (start, end)| {
        // Vertical
        if start.0 == end.0 {
            for i in start.1.min(end.1)..=start.1.max(end.1) {
                let key = (start.0, i);
                *freq.entry(key).or_insert(0) += 1;
            }
        // Horizontal
        } else if start.1 == end.1 {
            for i in start.0.min(end.0)..=start.0.max(end.0) {
                let key = (i, start.1);
                *freq.entry(key).or_insert(0) += 1;
            }
        // Bottom left to top right
        } else if start.1 < end.1 {
            for i in 0..=(end.0 - start.0) {
                let key = (start.0 + i, start.1 + i);
                *freq.entry(key).or_insert(0) += 1;
            }
        // Top left to bottom right
        } else if start.1 > end.1 {
            for i in 0..=(end.0 - start.0) {
                let key = (start.0 + i, start.1 - i);
                *freq.entry(key).or_insert(0) += 1;
            }
        } else {
            panic!()
        }
        freq
    });

    freq.into_iter().filter(|(_key, value)| value > &1).count()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT: &[u8; 109] = b"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 5);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 12);
    }
}
