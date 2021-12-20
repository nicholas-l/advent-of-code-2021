use std::{collections::HashSet, io::BufRead};

struct Image {
    is_light: bool,
    values: HashSet<(isize, isize)>,
}

impl Image {
    fn get_surrounding(&self, r: isize, c: isize, bounds: &(isize, isize, isize, isize)) -> usize {
        let dirs = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        dirs.iter()
            .map(|(dr, dc)| (r + dr, c + dc))
            .map(|pos| {
                (
                    bounds.0.max(pos.0).min(bounds.1),
                    bounds.2.max(pos.1).min(bounds.3),
                )
            })
            .map(|pos| match self.values.get(&pos) {
                Some(_) => self.is_light as usize,
                None => !self.is_light as usize,
            })
            .fold(0, |acc, curr| acc * 2 + curr)
    }
}

fn enhance(image: &mut Image, iep: &[char], flips: bool, should_print: bool) {
    let bounds = (
        *image.values.iter().map(|(r, _c)| r).min().unwrap() - 3,
        *image.values.iter().map(|(r, _c)| r).max().unwrap() + 3,
        *image.values.iter().map(|(_r, c)| c).min().unwrap() - 3,
        *image.values.iter().map(|(_r, c)| c).max().unwrap() + 3,
    );
    let is_going_to_be_positive = if flips { !image.is_light } else { true };
    image.values = ((bounds.0 - 3)..=(bounds.1 + 3))
        .flat_map(|r| {
            ((bounds.2 - 3)..=(bounds.3 + 3))
                .filter(|&c| {
                    if is_going_to_be_positive {
                        iep[image.get_surrounding(r, c, &bounds)] == '#'
                    } else {
                        iep[image.get_surrounding(r, c, &bounds)] == '.'
                    }
                })
                .map(move |c| (r, c))
                .collect::<Vec<_>>()
        })
        .collect();
    image.is_light = is_going_to_be_positive;

    // println!("Round: {}", i);
    let positive = if image.is_light { '#' } else { '.' };
    let negative = if image.is_light { '.' } else { '#' };

    if should_print {
        for r in (bounds.0 - 3)..=(bounds.1 + 3) {
            for c in (bounds.2 - 3)..=(bounds.3 + 3) {
                print!(
                    "{}",
                    match image.values.get(&(r, c)) {
                        Some(_) => positive,
                        None => negative,
                    }
                )
            }
            println!("");
        }
    }
}

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();
    let mut sections = buf.split("\n\n");

    let original_iep: Vec<_> = sections
        .next()
        .unwrap()
        .chars()
        .filter(|c| c != &'\n')
        .collect();

    let mut image = Image {
        is_light: true,
        values: sections
            .next()
            .unwrap()
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_col, c)| match c {
                        '#' => true,
                        '.' => false,
                        x => panic!("{}", x),
                    })
                    .map(move |(col, _c)| (row as isize, col as isize))
            })
            .collect(),
    };

    // if 0 is # then invert

    let iep = original_iep.clone();

    let flips = dbg!(iep[0] == '#' && iep[511] == '.');

    for _i in 0..2 {
        enhance(&mut image, &iep, flips, false);
    }

    image.values.len()
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();
    let mut sections = buf.split("\n\n");

    let original_iep: Vec<_> = sections
        .next()
        .unwrap()
        .chars()
        .filter(|c| c != &'\n')
        .collect();

    let mut image = Image {
        is_light: true,
        values: sections
            .next()
            .unwrap()
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_col, c)| match c {
                        '#' => true,
                        '.' => false,
                        x => panic!("{}", x),
                    })
                    .map(move |(col, _c)| (row as isize, col as isize))
            })
            .collect(),
    };

    // if 0 is # then invert

    let iep = original_iep.clone();

    let flips = dbg!(iep[0] == '#' && iep[511] == '.');

    for _i in 0..50 {
        enhance(&mut image, &iep, flips, false);
    }

    image.values.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        let input = b"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        assert_eq!(star_one(Cursor::new(input)), 35);
    }

    #[test]
    fn test_star_two() {
        let input = b"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        assert_eq!(star_two(Cursor::new(input)), 3351);
    }
}
