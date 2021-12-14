use std::{cmp::Ordering, collections::HashSet, io::BufRead};

type Position = (usize, usize);

fn fold(data: HashSet<Position>, dir: &str, value: usize) -> HashSet<Position> {
    match dir {
        "y" => {
            // split the data
            let (mut data, to_fold) =
                data.into_iter()
                    .fold((HashSet::new(), Vec::new()), |(mut hm, mut vec), key| {
                        match key.1.cmp(&value) {
                            Ordering::Less => {
                                hm.insert(key);
                            }
                            Ordering::Equal => {}
                            Ordering::Greater => {
                                vec.push(key);
                            }
                        }
                        (hm, vec)
                    });

            // Run the fold
            for key in to_fold {
                let distance_to_fold = key.1 - value;
                let new_key = (key.0, value - distance_to_fold);
                data.insert(new_key);
            }

            data
        }
        "x" => {
            // split the data
            let (mut data, to_fold) =
                data.into_iter()
                    .fold((HashSet::new(), Vec::new()), |(mut hm, mut vec), key| {
                        match key.0.cmp(&value) {
                            Ordering::Less => {
                                hm.insert(key);
                            }
                            Ordering::Equal => {}
                            Ordering::Greater => {
                                vec.push(key);
                            }
                        }
                        (hm, vec)
                    });

            for key in to_fold {
                let distance_to_fold = key.0 - value;
                let new_key = (value - distance_to_fold, key.1);
                data.insert(new_key);
            }
            data
        }
        _ => panic!(),
    }
}

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut buf = String::new();

    let _res = input.read_to_string(&mut buf);

    let (board, folds) = buf.split_once("\n\n").unwrap();

    let mut data = board.lines().fold(HashSet::new(), |mut hm, line| {
        let (x, y) = line.split_once(',').unwrap();
        let key = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
        hm.insert(key);
        hm
    });

    let line = folds.lines().next().unwrap();
    let (dir, value) = line
        .strip_prefix("fold along ")
        .unwrap()
        .split_once('=')
        .unwrap();

    let value = value.parse::<usize>().unwrap();

    data = fold(data, dir, value);

    data.len()
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut buf = String::new();

    let _res = input.read_to_string(&mut buf);

    let (board, folds) = buf.split_once("\n\n").unwrap();

    let mut data = board.lines().fold(HashSet::new(), |mut hm, line| {
        let (x, y) = line.split_once(',').unwrap();
        let key = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
        hm.insert(key);
        hm
    });

    for line in folds.lines() {
        let (dir, value) = line
            .strip_prefix("fold along ")
            .unwrap()
            .split_once('=')
            .unwrap();

        let value = value.parse::<usize>().unwrap();

        data = fold(data, dir, value);
    }

    let max_0 = data.iter().max_by_key(|k| k.0).unwrap().0;
    let max_1 = data.iter().max_by_key(|k| k.1).unwrap().1;

    let should_print = false;
    if should_print {
        for k in 0..=max_1 {
            for i in 0..=max_0 {
                let c = if data.contains(&(i, k)) { '#' } else { '.' };
                print!("{}", c);
            }
            println!();
        }
    }

    data.len()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT: &[u8] = b"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 17);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 16);
    }
}
