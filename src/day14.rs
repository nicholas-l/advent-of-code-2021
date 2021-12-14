use std::{collections::HashMap, io::BufRead};

use itertools::Itertools;

///
/// This stores the counts of each of the combinations adjacent pairs that are in the template.
/// Loops through the polymer rules for each timestep and computes the new adjacent pairs HashMap
fn star(mut input: impl BufRead, iterations: usize) -> usize {
    let mut buf = String::new();

    let _res = input.read_to_string(&mut buf);

    let (start_string, mapping) = buf.split_once("\n\n").unwrap();

    let rules = mapping.lines().fold(HashMap::new(), |mut hm, line| {
        let (inputs, outputs) = line.split_once(" -> ").unwrap();

        let inputs: Vec<_> = inputs.chars().collect();
        let outputs: Vec<_> = outputs.chars().collect();

        hm.insert((inputs[0], inputs[1]), outputs[0]);
        hm
    });

    let mut polymer =
        start_string
            .chars()
            .tuple_windows()
            .fold(HashMap::new(), |mut hm, (c1, c2)| {
                *hm.entry((c1, c2)).or_insert(0) += 1_u128;
                hm
            });

    for _i in 0..iterations {
        let mut new_polymer = HashMap::new();
        for (input, output) in &rules {
            if let Some(count) = polymer.get(input) {
                *new_polymer.entry((input.0, *output)).or_insert(0) += count;
                *new_polymer.entry((*output, input.1)).or_insert(0) += count;
            }
        }
        polymer = new_polymer;
    }

    let freq = polymer.iter().fold(HashMap::new(), |mut hm, (k, v)| {
        *hm.entry(k.0).or_insert(0) += v / 2;
        *hm.entry(k.1).or_insert(0) += v / 2;
        hm
    });

    let max = freq.iter().max_by_key(|x| x.1).unwrap().1;
    let min = freq.iter().min_by_key(|x| x.1).unwrap().1;

    (max - min) as usize
}

/// Completes in 185.08 us
pub fn star_one(input: impl BufRead) -> usize {
    star(input, 10)
}

/// Completes in 577.48 us
pub fn star_two(input: impl BufRead) -> usize {
    star(input, 40)
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
