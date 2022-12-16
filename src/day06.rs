use std::{collections::HashMap, io::BufRead};

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let mut fish: Vec<u8> = buf.split(',').map(|x| x.parse::<u8>().unwrap()).collect();

    for _ in 0..80 {
        let mut new_fish = Vec::new();
        for f in fish.iter_mut() {
            if *f == 0 {
                *f = 7;
                new_fish.push(8);
            }
            *f -= 1;
        }
        fish.extend(new_fish);
    }
    fish.len()
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let fish = buf
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let mut ages: HashMap<u8, u128> = HashMap::new();

    for f in fish {
        *ages.entry(f).or_insert(0) += 1;
    }

    for _ in 0..256 {
        let mut new_ages = HashMap::new();
        for (age, amount) in ages {
            if age == 0 {
                *new_ages.entry(6).or_insert(0) += amount;
                *new_ages.entry(8).or_insert(0) += amount;
            } else {
                *new_ages.entry(age - 1).or_insert(0) += amount;
            }
        }
        ages = new_ages;
    }
    ages.into_values().sum::<u128>() as usize
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT: &[u8; 9] = b"3,4,3,1,2";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 5934);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 26984457539);
    }
}
