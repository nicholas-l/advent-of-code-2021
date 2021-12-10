use std::io::BufRead;

#[derive(Debug)]
enum Status {
    Corrupted(char),
    Incomplete(Vec<char>),
}

pub fn star_one(input: impl BufRead) -> usize {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let chars = line.chars();

            let mut stack = Vec::new();

            for c in chars {
                match (stack.last(), c) {
                    (_, '(' | '[' | '{' | '<') => stack.push(c),
                    (Some('('), ')') => {
                        stack.pop();
                    }
                    (Some('['), ']') => {
                        stack.pop();
                    }
                    (Some('{'), '}') => {
                        stack.pop();
                    }
                    (Some('<'), '>') => {
                        stack.pop();
                    }
                    (_, _) => return Status::Corrupted(c),
                }
            }
            if !stack.is_empty() {
                return Status::Incomplete(stack);
            }
            unreachable!()
        })
        .map(|s| match s {
            Status::Corrupted(')') => 3,
            Status::Corrupted(']') => 57,
            Status::Corrupted('}') => 1197,
            Status::Corrupted('>') => 25137,
            Status::Corrupted(c) => panic!("{}", c),
            Status::Incomplete(_) => 0,
        })
        .sum()
}

pub fn star_two(input: impl BufRead) -> usize {
    let mut scores: Vec<usize> = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let chars = line.chars();

            let mut stack = Vec::new();

            for c in chars {
                match (stack.last(), c) {
                    (_, '(' | '[' | '{' | '<') => stack.push(c),
                    (Some('('), ')') => {
                        stack.pop();
                    }
                    (Some('['), ']') => {
                        stack.pop();
                    }
                    (Some('{'), '}') => {
                        stack.pop();
                    }
                    (Some('<'), '>') => {
                        stack.pop();
                    }
                    (_, _) => return Status::Corrupted(c),
                }
            }
            if !stack.is_empty() {
                return Status::Incomplete(stack);
            }
            unreachable!()
        })
        .filter(|s| matches!(s, Status::Incomplete(_)))
        .map(|s| match s {
            Status::Corrupted(_c) => panic!(),
            Status::Incomplete(stack) => {
                let mut score = 0;
                for c in stack.into_iter().rev() {
                    score *= 5;
                    score += match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!(),
                    };
                }
                score
            }
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT: &[u8] = b"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 26397);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 288957);
    }
}
