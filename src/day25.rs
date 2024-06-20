use std::{fmt::Display, io::BufRead};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cucumber {
    East,
    South,
}

#[derive(Debug, PartialEq, Eq)]
struct Map(Vec<Vec<Option<Cucumber>>>);

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for x in row {
                let c = match x {
                    Some(Cucumber::East) => '>',
                    Some(Cucumber::South) => 'v',
                    None => '.',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn star_one(input: impl BufRead) -> usize {
    let mut map = Map(input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| match c {
                    '.' => None,
                    '>' => Some(Cucumber::East),
                    'v' => Some(Cucumber::South),
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect());
    // println!("{}", map);

    let mut i = 0;
    loop {
        i += 1;

        let new_map = Map(map
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .map(|(j, x)| {
                        match (
                            row[(j as isize - 1).rem_euclid(row.len() as isize) as usize].as_ref(),
                            x,
                            row[(j + 1).rem_euclid(row.len())].as_ref(),
                        ) {
                            (Some(Cucumber::East), None, _) => Some(Cucumber::East), // move left
                            (_, Some(Cucumber::East), None) => None,
                            (_, x, _) => x.clone(),
                        }
                    })
                    .collect()
            })
            .collect());
        // println!("Partial");
        // println!("{}", map2);

        let new_map = Map(new_map
            .0
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, x)| {
                        match (
                            new_map.0[(i as isize - 1).rem_euclid(map.0.len() as isize) as usize]
                                [j]
                                .as_ref(),
                            x,
                            new_map.0[(i + 1).rem_euclid(map.0.len())][j].as_ref(),
                        ) {
                            (Some(Cucumber::South), None, _) => Some(Cucumber::South),
                            (_, Some(Cucumber::South), None) => None,
                            (_, x, _) => x.clone(),
                        }
                    })
                    .collect()
            })
            .collect());
        // println!("{}", i);
        // println!("{}", map3);
        if map == new_map {
            break;
        }
        map = new_map
    }

    i
}

pub fn star_two(_input: impl BufRead) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        let input = b"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        assert_eq!(star_one(Cursor::new(input)), 58);
    }

    #[test]
    fn test_star_two() {
        let input = b"";
        assert_eq!(star_two(Cursor::new(input)), 0);
    }
}
