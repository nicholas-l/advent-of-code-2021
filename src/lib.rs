use std::{
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

type DayFn = fn(Box<dyn BufRead>) -> usize;

pub fn get_day(day: usize) -> (DayFn, DayFn, PathBuf) {
    match day {
        1 => {
            use day01::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day01.txt"),
            )
        }

        2 => {
            use day02::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day02.txt"),
            )
        }

        3 => {
            use day03::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day03.txt"),
            )
        }

        4 => {
            use day04::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day04.txt"),
            )
        }

        5 => {
            use day05::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day05.txt"),
            )
        }

        6 => {
            use day06::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day06.txt"),
            )
        }

        7 => {
            use day07::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day07.txt"),
            )
        }

        8 => {
            use day08::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day08.txt"),
            )
        }

        9 => {
            use day09::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day09.txt"),
            )
        }

        10 => {
            use day10::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day10.txt"),
            )
        }

        11 => {
            use day11::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day11.txt"),
            )
        }

        12 => {
            use day12::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day12.txt"),
            )
        }

        13 => {
            use day13::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day13.txt"),
            )
        }

        14 => {
            use day14::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day14.txt"),
            )
        }

        x => {
            unimplemented!("Have not implemented day {}", x);
        }
    }
}

pub fn get_days() -> impl Iterator<Item = usize> {
    1..=14
}

pub fn get_data(filepath: &PathBuf) -> Box<dyn BufRead> {
    let f = fs::File::open(filepath).unwrap();
    let input = BufReader::new(f);
    Box::new(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_complete() {
        let (star_one, star_two, filepath) = get_day(1);
        assert_eq!(star_one(get_data(&filepath)), 1532);

        assert_eq!(star_two(get_data(&filepath)), 1571);
    }

    #[test]
    fn day02_complete() {
        let (star_one, star_two, filepath) = get_day(2);
        assert_eq!(star_one(get_data(&filepath)), 1250395);

        assert_eq!(star_two(get_data(&filepath)), 1451210346);
    }

    #[test]
    fn day03_complete() {
        let (star_one, star_two, filepath) = get_day(3);
        assert_eq!(star_one(get_data(&filepath)), 3813416);

        assert_eq!(star_two(get_data(&filepath)), 2990784);
    }

    #[test]
    fn day04_complete() {
        let (star_one, star_two, filepath) = get_day(4);
        assert_eq!(star_one(get_data(&filepath)), 60368);

        assert_eq!(star_two(get_data(&filepath)), 17435);
    }

    #[test]
    fn day05_complete() {
        let (star_one, star_two, filepath) = get_day(5);
        assert_eq!(star_one(get_data(&filepath)), 5124);

        assert_eq!(star_two(get_data(&filepath)), 19771);
    }

    #[test]
    fn day06_complete() {
        let (star_one, star_two, filepath) = get_day(6);
        assert_eq!(star_one(get_data(&filepath)), 387413);

        assert_eq!(star_two(get_data(&filepath)), 1738377086345);
    }

    #[test]
    fn day07_complete() {
        let (star_one, star_two, filepath) = get_day(7);
        assert_eq!(star_one(get_data(&filepath)), 352707);

        assert_eq!(star_two(get_data(&filepath)), 95519693);
    }

    #[test]
    fn day08_complete() {
        let (star_one, star_two, filepath) = get_day(8);
        assert_eq!(star_one(get_data(&filepath)), 530);

        assert_eq!(star_two(get_data(&filepath)), 1051087);
    }

    #[test]
    fn day09_complete() {
        let (star_one, star_two, filepath) = get_day(9);
        assert_eq!(star_one(get_data(&filepath)), 539);

        assert_eq!(star_two(get_data(&filepath)), 736920);
    }

    #[test]
    fn day10_complete() {
        let (star_one, star_two, filepath) = get_day(10);
        assert_eq!(star_one(get_data(&filepath)), 411471);

        assert_eq!(star_two(get_data(&filepath)), 3122628974);
    }

    #[test]
    fn day11_complete() {
        let (star_one, star_two, filepath) = get_day(11);
        assert_eq!(star_one(get_data(&filepath)), 1729);

        assert_eq!(star_two(get_data(&filepath)), 237);
    }

    #[test]
    fn day12_complete() {
        let (star_one, star_two, filepath) = get_day(12);
        assert_eq!(star_one(get_data(&filepath)), 4378);

        assert_eq!(star_two(get_data(&filepath)), 133621);
    }

    #[test]
    fn day13_complete() {
        let (star_one, star_two, filepath) = get_day(13);
        assert_eq!(star_one(get_data(&filepath)), 818);

        assert_eq!(star_two(get_data(&filepath)), 101);
    }

    #[test]
    fn day14_complete() {
        let (star_one, star_two, filepath) = get_day(14);
        assert_eq!(star_one(get_data(&filepath)), 2408);

        assert_eq!(star_two(get_data(&filepath)), 2651311098752);
    }
}
