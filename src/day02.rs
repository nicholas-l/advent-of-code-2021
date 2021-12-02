use std::{io::BufRead, str::FromStr};

enum Operation {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, x) = s.split_once(" ").unwrap();
        match op {
            "forward" => Ok(Operation::Forward(x.parse::<i32>().unwrap())),
            "up" => Ok(Operation::Up(x.parse::<i32>().unwrap())),
            "down" => Ok(Operation::Down(x.parse::<i32>().unwrap())),
            x => panic!("{}", x),
        }
    }
}

pub fn star_one(input: impl BufRead) -> usize {
    let mut depth = 0;
    let mut pos = 0;
    for op in input
        .lines()
        .map(|x| x.unwrap().parse::<Operation>().unwrap())
    {
        match op {
            Operation::Forward(x) => pos += x,
            Operation::Up(y) => depth -= y,
            Operation::Down(y) => depth += y,
        };
    }
    (depth * pos) as usize
}

pub fn star_two(input: impl BufRead) -> usize {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for op in input
        .lines()
        .map(|x| x.unwrap().parse::<Operation>().unwrap())
    {
        match op {
            Operation::Forward(x) => {
                depth += x * aim;
                pos += x
            }
            Operation::Up(x) => aim -= x,
            Operation::Down(x) => aim += x,
        }
    }
    (depth * pos) as usize
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"forward 5
down 5
forward 8
up 3
down 8
forward 2"
            )),
            150
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"forward 5
down 5
forward 8
up 3
down 8
forward 2"
            )),
            900
        );
    }
}
