use std::{collections::HashSet, io::BufRead};

fn get_next_9(data: &[Vec<usize>]) -> Option<(usize, usize)> {
    for (i, row) in data.iter().enumerate() {
        for (j, x) in row.iter().enumerate() {
            if x > &9 {
                return Some((i, j));
            }
        }
    }
    None
}

fn step(data: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut new_data = data
        .iter()
        .map(|row| row.iter().map(|x| x + 1).collect())
        .collect::<Vec<Vec<usize>>>();
    let dirs: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut flashed = HashSet::new();

    while let Some((i, j)) = get_next_9(&new_data) {
        if new_data[i][j] > 9 {
            new_data[i][j] = 0;
            flashed.insert((i, j));
            for (di, dj) in dirs {
                let pos = (i as isize + di, j as isize + dj);
                if pos.0 >= 0
                    && pos.0 < new_data.len() as isize
                    && pos.1 >= 0
                    && (pos.1 as usize) < new_data[pos.0 as usize].len()
                {
                    let position = (pos.0 as usize, pos.1 as usize);
                    if !flashed.contains(&position) {
                        new_data[position.0][position.1] += 1;
                    }
                }
            }
        }
    }

    new_data
}

fn is_all_zeros(data: &[Vec<usize>]) -> bool {
    data.iter().all(|row| row.iter().all(|&x| x == 0))
}

pub fn star_one(input: impl BufRead) -> usize {
    let mut data: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let mut flashes = 0;
    for _ in 0..100 {
        data = step(data);
        flashes += data
            .iter()
            .flat_map(|row| row.iter().filter(|&&x| x == 0))
            .count();
    }
    flashes
}

pub fn star_two(input: impl BufRead) -> usize {
    let mut data: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let mut i = 0;
    while !is_all_zeros(&data) {
        data = step(data);
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two, step};
    use std::io::{BufRead, Cursor};

    const INPUT: &[u8] = b"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 1656);
    }

    #[test]
    fn test_step_one() {
        let input = Box::new(Cursor::new(INPUT)) as Box<dyn BufRead>;
        let data = input
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        let expected = b"6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637";

        let input = Box::new(Cursor::new(expected)) as Box<dyn BufRead>;
        let expected: Vec<Vec<usize>> = input
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        assert_eq!(step(data), expected);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 195);
    }
}
