use std::env;
use std::fs;

use std::io::BufReader;

use advent_of_code_2020::*;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let days = if args.len() == 1 && args[0] == "all" {
        (1..=25).map(|x| x.to_string()).collect()
    } else {
        args
    };
    for day in days {
        println!("Day {}:", day);

        let (star_one, star_two, filename) = get_day(day.parse::<usize>().unwrap());
        let f = fs::File::open(&filename).unwrap();
        let input = BufReader::new(f);
        println!("Star One: {}", star_one(Box::new(input)));
        let f = fs::File::open(&filename).unwrap();
        let input = BufReader::new(f);
        println!("Star Two: {}", star_two(Box::new(input)));
    }
}
