

use std::{cmp::Ordering, collections::{HashSet, LinkedList, HashMap}, io::BufRead};

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut buf = String::new();

    let _res = input.read_to_string(&mut buf);

    let (start_string, mapping) = buf.split_once("\n\n").unwrap();

    let mapping = mapping.lines().fold(HashMap::new(), |mut hm, line| {
        let (inputs, outputs) = line.split_once(" -> ").unwrap();

        let inputs: Vec<_> = inputs.chars().collect();
        let outputs: Vec<_> = outputs.chars().collect();

        hm.insert((inputs[0], inputs[1]), outputs[0]);
        hm
    });

    let mut start: LinkedList<char> = start_string.chars().collect();

    for _i in 0..10 {
        let mut cursor = start.cursor_front_mut();
        cursor.move_next();
        while !matches!(cursor.current(), None) {
            let key = (*cursor.peek_prev().unwrap(), *cursor.current().unwrap());
            if let Some(v) = mapping.get(&key) {
                cursor.insert_before(*v);
            }
            cursor.move_next();
        }
    }

    let freq = start.iter().fold(HashMap::new(), |mut hm, c| {
        *hm.entry(c).or_insert(0) += 1;
        hm
    });

    let max = freq.iter().max_by_key(|x| x.1).unwrap().1;
    let min = freq.iter().min_by_key(|x| x.1).unwrap().1;

    max - min

}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut buf = String::new();

    let _res = input.read_to_string(&mut buf);

    let (start_string, mapping) = buf.split_once("\n\n").unwrap();

    let mapping = mapping.lines().fold(HashMap::new(), |mut hm, line| {
        let (inputs, outputs) = line.split_once(" -> ").unwrap();

        let inputs: Vec<_> = inputs.chars().collect();
        let outputs: Vec<_> = outputs.chars().collect();

        hm.insert((inputs[0], inputs[1]), outputs[0]);
        hm
    });

    let mut start: LinkedList<char> = start_string.chars().collect();

    for _i in 0..40 {
        let mut cursor = start.cursor_front_mut();
        cursor.move_next();
        while !matches!(cursor.current(), None) {
            let key = (*cursor.peek_prev().unwrap(), *cursor.current().unwrap());
            if let Some(v) = mapping.get(&key) {
                cursor.insert_before(*v);
            }
            cursor.move_next();
        }
    }

    let freq = start.iter().fold(HashMap::new(), |mut hm, c| {
        *hm.entry(c).or_insert(0) += 1;
        hm
    });

    let max = freq.iter().max_by_key(|x| x.1).unwrap().1;
    let min = freq.iter().min_by_key(|x| x.1).unwrap().1;

    max - min
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT: &[u8] = b"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 1588);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 2188189693529);
    }
}
