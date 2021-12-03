use std::{cmp::Ordering, collections::HashMap, io::BufRead};

pub fn star_one(input: impl BufRead) -> usize {
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect();

    let bit_length = lines[0].len();

    let most_common = (0..bit_length)
        .map(|bit| {
            let mut freq = HashMap::new();
            for line in &lines {
                *freq.entry(line[bit]).or_insert(0) += 1;
            }
            freq.into_iter().max_by_key(|(_c, x)| *x).unwrap().0
        })
        .collect::<String>();
    let gamma = usize::from_str_radix(&most_common, 2).unwrap();
    gamma * (gamma ^ (2_i32.pow(bit_length as u32) - 1) as usize)
}

fn get_value(mut possible: Vec<Vec<char>>, check: impl Fn(usize, usize) -> char) -> usize {
    let bit_length = possible[0].len();
    for bit in 0..bit_length {
        if possible.len() == 1 {
            break;
        }
        let mut freq = HashMap::new();
        for line in &possible {
            *freq.entry(line[bit]).or_insert(0) += 1;
        }

        let common = check(freq[&'1'], freq[&'0']);
        let mut i = 0;
        while i < possible.len() {
            if possible[i][bit] != common {
                possible.remove(i);
            } else {
                i += 1;
            }
        }
    }
    let oxygen = usize::from_str_radix(&possible[0].iter().collect::<String>(), 2).unwrap();
    oxygen
}

pub fn star_two(input: impl BufRead) -> usize {
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect();

    let oxygen = get_value(lines.clone(), |a, b| match a.cmp(&b) {
        Ordering::Less => '0',
        Ordering::Equal => '1',
        Ordering::Greater => '1',
    });

    let co2 = get_value(lines, |a, b| match a.cmp(&b) {
        Ordering::Less => '1',
        Ordering::Equal => '0',
        Ordering::Greater => '0',
    });

    oxygen * co2
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            )),
            198
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            )),
            230
        );
    }
}
