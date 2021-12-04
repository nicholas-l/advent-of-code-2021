use std::{io::BufRead, str::FromStr};

#[derive(Debug, Clone)]
struct Board(Vec<Vec<(usize, bool)>>);

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = s
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|v| (v.parse::<usize>().unwrap(), false))
                    .collect()
            })
            .collect();
        Ok(Board(t))
    }
}

impl Board {
    fn mark(&mut self, num: usize) -> bool {
        let mut changed = None;
        for (i, row) in self.0.iter_mut().enumerate() {
            for (j, x) in row.iter_mut().enumerate() {
                if x.0 == num {
                    x.1 = true;
                    // loop through checking horizontal & vertical
                    changed = Some((i, j));
                }
            }
        }
        if let Some((i, j)) = changed {
            if self.0[i].iter().all(|x| x.1) {
                return true;
            }
            if self.0.iter().all(|row| row[j].1) {
                return true;
            }
        }
        false
    }
}

fn print(board: &Board) {
    for row in &board.0 {
        println!("{:?}", row);
    }
}

fn get_winning_board(mut boards: Vec<Board>, numbers: Vec<usize>) -> (Board, usize) {
    for number in numbers {
        for board in boards.iter_mut() {
            if board.mark(number) {
                return (board.clone(), number);
            }
        }
    }
    unreachable!()
}

fn drain<T>(possible: &mut Vec<T>, should_remove: impl Fn(&mut T) -> bool) {
    let mut i = 0;
    while i < possible.len() {
        if should_remove(&mut possible[i]) {
            possible.remove(i);
        } else {
            i += 1;
        }
    }
}

fn get_least_winning(mut boards: Vec<Board>, numbers: Vec<usize>) -> (Board, usize) {
    for number in numbers {
        if boards.len() > 1 {
            drain(&mut boards, |board| board.mark(number));
        } else if boards[0].mark(number) {
            return (boards[0].clone(), number);
        }
    }

    unreachable!()
}

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut buf = String::new();

    let _res = input.read_to_string(&mut buf);

    let mut sections = buf.split("\n\n");

    let t: Vec<usize> = sections
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<usize>().unwrap_or_else(|_| panic!("{}", c)))
        .collect();

    let boards: Vec<Board> = sections
        .map(|section| section.parse::<Board>().unwrap())
        .collect();

    let (winning_board, number) = get_winning_board(boards, t);

    dbg!(&winning_board);

    number
        * winning_board
            .0
            .iter()
            .flatten()
            .filter(|(_value, marked)| !marked)
            .map(|(value, _marked)| value)
            .sum::<usize>()
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut buf = String::new();

    let _res = input.read_to_string(&mut buf);

    let mut sections = buf.split("\n\n");

    let t: Vec<usize> = sections
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<usize>().unwrap_or_else(|_| panic!("{}", c)))
        .collect();

    let boards: Vec<Board> = sections
        .map(|section| section.parse::<Board>().unwrap())
        .collect();

    let (winning_board, number) = get_least_winning(boards, t);

    print(&winning_board);
    dbg!(number);

    number
        * winning_board
            .0
            .iter()
            .flatten()
            .filter(|(_value, marked)| !marked)
            .map(|(value, _marked)| value)
            .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT: &[u8; 292] =
        b"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 4512);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 1924);
    }
}
