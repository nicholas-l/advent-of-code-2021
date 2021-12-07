use std::io::BufRead;

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let mut data = buf
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    data.sort_unstable();

    let mid = data.len() / 2;

    let median = data[mid];

    data.iter()
        .map(|&x| if x < median { median - x } else { x - median })
        .sum()
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let data = buf
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mean = (data.iter().sum::<usize>() as f64 / data.len() as f64).ceil() as usize;

    let mid: usize = data
        .iter()
        .map(|&x| {
            let distance = if mean < x { x - mean } else { mean - x };
            (1..=distance).sum::<usize>()
        })
        .sum();
    let mean_left = mean - 1;
    let left = data
        .iter()
        .map(|&x| {
            let distance = if mean_left < x {
                x - mean_left
            } else {
                mean_left - x
            };
            (1..=distance).sum::<usize>()
        })
        .sum();
    let mean_right = mean - 1;
    let right = data
        .iter()
        .map(|&x| {
            let distance = if mean_right < x {
                x - mean_right
            } else {
                mean_right - x
            };
            (1..=distance).sum::<usize>()
        })
        .sum();
    mid.min(left).min(right)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT: &[u8; 21] = b"16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 37);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 168);
    }
}
