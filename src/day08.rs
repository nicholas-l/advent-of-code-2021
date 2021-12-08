use std::{
    collections::{BTreeSet, HashMap, HashSet},
    io::BufRead,
};

#[allow(dead_code)]
enum DisplayPosition {
    Top,
    TopLeft,
    BottomLeft,
    Bottom,
    TopRight,
    BottomRight,
    Middle,
}
/*
0: 6 Done
1: 2 Done
2: 5 Done
3: 5 Done
4: 4 Done
5: 5 Done
6: 6 Done
7: 3 Done
8: 7 Done
9: 6 Done
*/

// We dont need to know the letters corresponding to the position so just need to know the combinations.
fn guess_inputs_v2(mut inputs: Vec<BTreeSet<char>>) -> HashMap<BTreeSet<char>, usize> {
    let mut positions = HashMap::new();

    let position_1 = inputs.iter().position(|x| x.len() == 2).unwrap();
    let inputs_for_1 = inputs.remove(position_1);
    positions.insert(inputs_for_1.clone(), 1);

    let position_7 = inputs.iter().position(|x| x.len() == 3).unwrap();
    let inputs_for_7 = inputs.remove(position_7);
    positions.insert(inputs_for_7, 7);

    let position_4 = inputs.iter().position(|x| x.len() == 4).unwrap();
    let inputs_for_4 = inputs.remove(position_4);
    positions.insert(inputs_for_4.clone(), 4);

    let position_8 = inputs.iter().position(|x| x.len() == 7).unwrap();
    let inputs_for_8 = inputs.remove(position_8);
    positions.insert(inputs_for_8, 8);

    let position_3 = inputs
        .iter()
        .position(|x| x.len() == 5 && x.is_superset(&inputs_for_1))
        .unwrap();
    let inputs_for_3 = inputs.remove(position_3);
    positions.insert(inputs_for_3, 3);

    let position_9 = inputs
        .iter()
        .position(|x| x.len() == 6 && x.is_superset(&inputs_for_4))
        .unwrap();
    let inputs_for_9 = inputs.remove(position_9);
    positions.insert(inputs_for_9, 9);

    let position_6 = inputs
        .iter()
        .position(|x| x.len() == 6 && !x.is_superset(&inputs_for_1))
        .unwrap();
    let inputs_for_6 = inputs.remove(position_6);
    positions.insert(inputs_for_6.clone(), 6);

    let position_0 = inputs
        .iter()
        .position(|x| x.len() == 6 && x.is_superset(&inputs_for_1))
        .unwrap();
    let inputs_for_0 = inputs.remove(position_0);
    positions.insert(inputs_for_0, 0);

    let position_5 = inputs
        .iter()
        .position(|x| x.len() == 5 && x.symmetric_difference(&inputs_for_6).count() == 1)
        .unwrap();
    let inputs_for_5 = inputs.remove(position_5);
    positions.insert(inputs_for_5, 5);

    assert_eq!(inputs.len(), 1);
    let inputs_for_2 = inputs.remove(0);
    positions.insert(inputs_for_2, 2);

    positions
}

// First try, not complete
#[allow(dead_code)]
fn guess_inputs(inputs: Vec<HashSet<char>>) -> HashMap<char, DisplayPosition> {
    let mut positions = HashMap::new();

    // get Top by removing 1s from 7
    let inputs_for_1 = inputs.iter().find(|x| x.len() == 2).unwrap();
    let inputs_for_7 = inputs.iter().find(|x| x.len() == 3).unwrap();
    let inputs_for_8 = inputs.iter().find(|x| x.len() == 7).unwrap();

    let top = inputs_for_7.difference(inputs_for_1).next().unwrap();

    assert!(!positions.contains_key(top));

    positions.insert(*top, DisplayPosition::Top);

    let inputs_for_3 = inputs
        .iter()
        .find(|x| x.len() == 5 && !x.is_superset(inputs_for_1))
        .unwrap();
    let inputs_for_4 = inputs
        .iter()
        .find(|x| x.len() != 2 && !x.contains(top))
        .unwrap();

    let lefts = inputs_for_8
        .difference(inputs_for_3)
        .cloned()
        .collect::<HashSet<_>>();
    let bottom_left_list: Vec<_> = lefts.difference(inputs_for_4).collect();

    assert_eq!(bottom_left_list.len(), 1);
    assert!(!positions.contains_key(top));
    positions.insert(*bottom_left_list[0], DisplayPosition::BottomLeft);

    let top_left_list: Vec<_> = lefts.intersection(inputs_for_4).collect();

    assert_eq!(top_left_list.len(), 1);
    assert!(!positions.contains_key(top));
    positions.insert(*top_left_list[0], DisplayPosition::TopLeft);

    // Bottom

    let _top_bottom_left = inputs_for_3
        .difference(inputs_for_4)
        .cloned()
        .collect::<HashSet<_>>();

    let _top_bottom_left = inputs_for_3
        .difference(inputs_for_4)
        .cloned()
        .collect::<HashSet<_>>();

    positions
}

pub fn star_one(input: impl BufRead) -> usize {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (_input, output) = line.split_once('|').unwrap();
            output
                .split(' ')
                .filter(|display| matches!(display.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

pub fn star_two(input: impl BufRead) -> usize {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (input, output) = line.split_once('|').unwrap();
            let inputs = input
                .trim()
                .split(' ')
                .map(|i| i.chars().collect::<BTreeSet<_>>())
                .collect();

            let mapping = guess_inputs_v2(inputs);

            output
                .trim()
                .split(' ')
                .map(|i| i.chars().collect::<BTreeSet<_>>())
                .map(|display| mapping[&display])
                .fold(0, |acc, x| acc * 10 + x)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT: &[u8] =
        b"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 26);
    }

    #[test]
    fn test_guess_inputs() {
        let inputs =
            b"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(star_two(Cursor::new(inputs)), 5353);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 61229);
    }
}
