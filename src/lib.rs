use std::{
    io::BufRead,
    path::{Path, PathBuf},
};

pub mod day01;
mod day02;
mod day03;
mod day04;
pub mod day05;

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

        x => {
            unimplemented!("Have not implemented day {}", x);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::BufReader;

    fn get_data(filepath: &PathBuf) -> Box<dyn BufRead> {
        let f = fs::File::open(filepath).unwrap();
        let input = BufReader::new(f);
        Box::new(input)
    }

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
}
