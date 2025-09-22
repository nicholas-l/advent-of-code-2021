use itertools::Itertools;
use nom::{
    branch::alt,
    // see the "streaming/complete" paragraph lower for an explanation of these submodules
    character::complete::{char, digit1},
    combinator::map_res,
    sequence::{delimited, separated_pair},
    IResult,
    Parser,
};
use std::{collections::VecDeque, fmt::Display, io::BufRead, ops::Add, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailfishNumber {
    Value(usize),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl SnailfishNumber {
    fn value(&self) -> usize {
        match self {
            SnailfishNumber::Value(x) => *x,
            SnailfishNumber::Pair(left, right) => panic!("{:?} {:?}", left, right),
        }
    }
    fn magnitude(&self) -> usize {
        match self {
            SnailfishNumber::Value(x) => *x,
            SnailfishNumber::Pair(x, y) => x.magnitude() * 3 + y.magnitude() * 2,
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailfishNumber::Value(x) => {
                if *x > 9 {
                    let left = SnailfishNumber::Value(*x / 2);
                    let right = SnailfishNumber::Value(*x / 2 + *x % 2);
                    *self = SnailfishNumber::Pair(Box::new(left), Box::new(right));
                    true
                } else {
                    false
                }
            }
            SnailfishNumber::Pair(x, y) => x.split() || y.split(),
        }
    }

    //
    fn explode(&mut self) -> bool {
        // If x and y are values and depth > 4
        let mut stack = VecDeque::new();
        let mut last_value_node = None;
        let mut right_value: Option<usize> = None;
        let mut changed = false;

        stack.push_back((0, self));

        while let Some((depth, node)) = stack.pop_back() {
            // println!("Looking at: {:?}", node);
            if right_value.is_none() && depth > 3 {
                match node {
                    SnailfishNumber::Value(_v) => {
                        last_value_node = Some(node);
                    }
                    SnailfishNumber::Pair(ref left, ref right) => {
                        // stack.push_back((depth + 1, left));
                        // stack.push_back((depth + 1, right));
                        changed = true;
                        let l = left.value();
                        if let Some(SnailfishNumber::Value(x)) = last_value_node {
                            *x += l;
                        }
                        right_value = Some(right.value());
                        // println!("Set right value to: {:?}", right_value);
                        *node = SnailfishNumber::Value(0);
                    }
                }
            } else if !changed || right_value.is_some() {
                match node {
                    SnailfishNumber::Value(v) => {
                        if let Some(x) = right_value.take() {
                            *v += x;
                            return true;
                        } else {
                            last_value_node = Some(node);
                        }
                    }
                    SnailfishNumber::Pair(left, right) => {
                        // println!("Left: {:?}, right: {:?}", left, right);
                        stack.push_back((depth + 1, right));
                        stack.push_back((depth + 1, left));
                    }
                }
            }
        }
        changed
    }
}

fn parse_value(input: &str) -> IResult<&str, SnailfishNumber> {
    map_res(digit1, |v: &str| {
        v.parse::<usize>().map(SnailfishNumber::Value)
    })
    .parse(input)
}

fn parse_snailfish_pair(input: &str) -> IResult<&str, SnailfishNumber> {
    let inner = separated_pair(
        alt((parse_snailfish_pair, parse_value)),
        char(','),
        alt((parse_snailfish_pair, parse_value)),
    );
    let (rest, numbers) = delimited(char('['), inner, char(']')).parse(input)?;
    let value = SnailfishNumber::Pair(Box::new(numbers.0), Box::new(numbers.1));
    Ok((rest, value))
}

impl FromStr for SnailfishNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rest, snailfishnumber) = parse_snailfish_pair(s).unwrap();
        assert_eq!(rest.len(), 0);

        Ok(snailfishnumber)
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailfishNumber::Value(v) => write!(f, "{}", v),
            SnailfishNumber::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

impl Add for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut s = SnailfishNumber::Pair(Box::new(self), Box::new(rhs));

        while s.explode() || s.split() {}
        // println!("After add: {}", s);
        s
    }
}

pub fn star_one(input: impl BufRead) -> usize {
    let mut lines = input.lines().map(|line| {
        let line = line.unwrap();
        line.parse::<SnailfishNumber>().unwrap()
    });
    let total = lines.next().unwrap();
    lines.fold(total, |acc, curr| acc + curr).magnitude()
}

pub fn star_two(input: impl BufRead) -> usize {
    let numbers = input.lines().map(|line| {
        let line = line.unwrap();
        line.parse::<SnailfishNumber>().unwrap()
    });

    numbers
        .permutations(2)
        .map(|v| v[0].clone() + v[1].clone())
        .max_by_key(|v| v.magnitude())
        .unwrap()
        .magnitude()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            "[1,2]".parse::<SnailfishNumber>(),
            Ok(SnailfishNumber::Pair(
                Box::new(SnailfishNumber::Value(1)),
                Box::new(SnailfishNumber::Value(2))
            ))
        );

        assert_eq!(
            "[[1,2],3]".parse::<SnailfishNumber>(),
            Ok(SnailfishNumber::Pair(
                Box::new(SnailfishNumber::Pair(
                    Box::new(SnailfishNumber::Value(1)),
                    Box::new(SnailfishNumber::Value(2))
                )),
                Box::new(SnailfishNumber::Value(3))
            ))
        );
    }

    #[test]
    fn test_split() {
        let mut v = SnailfishNumber::Value(10);
        assert!(v.split());
        assert_eq!(
            v,
            SnailfishNumber::Pair(
                Box::new(SnailfishNumber::Value(5)),
                Box::new(SnailfishNumber::Value(5))
            )
        );

        let mut v = SnailfishNumber::Value(11);
        assert!(v.split());
        assert_eq!(
            v,
            SnailfishNumber::Pair(
                Box::new(SnailfishNumber::Value(5)),
                Box::new(SnailfishNumber::Value(6))
            )
        );
    }

    #[test]
    fn test_explode() {
        let mut v: SnailfishNumber = "[[[[[9,8],1],2],3],4]".parse().unwrap();
        let expected: SnailfishNumber = "[[[[0,9],2],3],4]".parse().unwrap();

        assert!(v.explode());
        assert_eq!(v, expected);

        let mut v: SnailfishNumber = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
        let expected: SnailfishNumber = "[7,[6,[5,[7,0]]]]".parse().unwrap();

        assert!(v.explode());
        assert_eq!(v, expected);

        let mut v: SnailfishNumber = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
        let expected: SnailfishNumber = "[7,[6,[5,[7,0]]]]".parse().unwrap();

        assert!(v.explode());
        assert_eq!(v, expected);

        let mut v: SnailfishNumber = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
        let expected: SnailfishNumber = "[[6,[5,[7,0]]],3]".parse().unwrap();

        assert!(v.explode());
        assert_eq!(v, expected);

        let mut v: SnailfishNumber = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
        let expected: SnailfishNumber = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();

        assert!(v.explode());
        assert_eq!(v, expected);

        let mut v: SnailfishNumber = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();
        let expected: SnailfishNumber = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse().unwrap();

        assert!(v.explode());
        assert_eq!(v, expected);
    }

    #[test]
    fn test_magnitude() {
        let v: SnailfishNumber = "[[1,2],[[3,4],5]]".parse().unwrap();

        assert_eq!(v.magnitude(), 143);
    }

    #[test]
    fn test_add() {
        let expected = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap();
        let a: SnailfishNumber = "[1,1]".parse().unwrap();
        let b: SnailfishNumber = "[2,2]".parse().unwrap();
        let c: SnailfishNumber = "[3,3]".parse().unwrap();
        let d: SnailfishNumber = "[4,4]".parse().unwrap();

        let res = a + b + c + d;
        assert_eq!(res, expected);

        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();

        let left: SnailfishNumber = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let right: SnailfishNumber = "[1,1]".parse().unwrap();
        let res = left + right;
        assert_eq!(res, expected);

        let expected = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
            .parse()
            .unwrap();

        let left: SnailfishNumber = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".parse().unwrap();
        let right: SnailfishNumber = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse().unwrap();
        let res = left + right;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_sum() {
        let expected = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse()
            .unwrap();
        let mut lines = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"
            .lines()
            .map(|l| l.parse::<SnailfishNumber>().unwrap());

        let total = lines.next().unwrap();
        let res = lines.fold(total, |acc, curr| acc + curr);

        assert_eq!(res, expected);
    }

    #[test]
    fn test_star_one() {
        let input = b"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(star_one(Cursor::new(input)), 4140);
    }

    #[test]
    fn test_star_two() {
        let input = b"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(star_two(Cursor::new(input)), 3993);
    }
}
