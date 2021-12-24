use rayon::prelude::*;
use std::{collections::HashMap, io::BufRead};

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

lazy_static! {
    static ref CACHE: Mutex<Option<Vec<(Alu, (usize, usize))>>> = Mutex::new(None);
}

enum Value {
    Var(char),
    Number(i8),
}

enum Instruction {
    Input(char),
    Add(char, Value),
    Multiply(char, Value),
    Divide(char, Value),
    Modulus(char, Value),
    Equal(char, Value),
}

fn get_location(variable: char) -> usize {
    match variable {
        'w' => 0,
        'x' => 1,
        'y' => 2,
        'z' => 3,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Alu {
    memory: [isize; 4],
}

impl Alu {
    fn new() -> Alu {
        Alu { memory: [0; 4] }
    }

    fn value(&self, value: &Value) -> isize {
        match value {
            Value::Var(variable) => self.memory[get_location(*variable)],
            Value::Number(v) => *v as isize,
        }
    }

    fn operation(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Input(_c) => {
                panic!()
            }
            Instruction::Add(a, b) => {
                *self.memory.get_mut(get_location(*a)).unwrap() += self.value(b);
            }
            Instruction::Multiply(a, b) => {
                *self.memory.get_mut(get_location(*a)).unwrap() *= self.value(b);
            }
            Instruction::Divide(a, b) => {
                *self.memory.get_mut(get_location(*a)).unwrap() /= self.value(b);
            }
            Instruction::Modulus(a, b) => {
                *self.memory.get_mut(get_location(*a)).unwrap() %= self.value(b);
            }
            Instruction::Equal(a, b) => {
                let v = self.value(b);
                if let Some(x) = self.memory.get_mut(get_location(*a)) {
                    *x = (v == *x) as isize;
                }
            }
        }
    }

    fn set_memory(&mut self, variable: char, value: isize) {
        let location = get_location(variable);
        self.memory[location] = value;
    }

    fn parse_instructions(input: impl BufRead) -> Vec<Instruction> {
        input
            .lines()
            .map(|line| {
                let line = dbg!(line.unwrap());
                let mut separated = line.split(' ');
                let ins = separated.next();
                let char1 = separated.next().unwrap().chars().next().unwrap();
                let char2 = separated.next().map(|s| {
                    if let Ok(x) = s.parse::<i8>() {
                        Value::Number(x)
                    } else {
                        Value::Var(s.chars().next().unwrap())
                    }
                });
                match (ins, char1, char2) {
                    (Some("inp"), c1, None) => Instruction::Input(c1),
                    (Some("add"), c1, Some(c2)) => Instruction::Add(c1, c2),
                    (Some("mul"), c1, Some(c2)) => Instruction::Multiply(c1, c2),
                    (Some("div"), c1, Some(c2)) => Instruction::Divide(c1, c2),
                    (Some("mod"), c1, Some(c2)) => Instruction::Modulus(c1, c2),
                    (Some("eql"), c1, Some(c2)) => Instruction::Equal(c1, c2),
                    (Some(x), _, _) => panic!("{}", x),
                    (None, _, _) => panic!("None value for instruction"),
                }
            })
            .collect::<Vec<_>>()
    }
}

fn cached_run(input: impl BufRead) -> Vec<(Alu, (usize, usize))> {
    let mut lock = CACHE.lock().unwrap();
    if let Some(x) = &*lock {
        return x.clone();
    }
    let instructions = Alu::parse_instructions(input);

    let computer: Alu = Alu::new();
    // A list of computers with their previous input (in base 10).
    let mut computers = vec![(computer, (0usize, 0usize))];

    let mut inputs_seen = 0;

    for instruction in &instructions {
        match instruction {
            Instruction::Input(c) => {
                // We have seen an input instruction so lets branch out the combinations of
                // computers
                computers = computers
                    .into_iter()
                    .fold(
                        HashMap::new(),
                        |mut computers: HashMap<Alu, (usize, usize)>,
                         (computer, (previous_min, previous_max))| {
                            for input in 1..=9 {
                                let mut new_computer = computer.clone();
                                new_computer.set_memory(*c, input);
                                let new_min = previous_min * 10 + input as usize;
                                let new_max = previous_max * 10 + input as usize;

                                computers
                                    .entry(new_computer)
                                    .and_modify(|e| {
                                        *e = (e.0.min(new_min), e.1.max(new_max));
                                    })
                                    .or_insert((new_min, new_max));
                            }
                            computers
                        },
                    )
                    .into_iter()
                    .collect();
                inputs_seen += 1;
                println!("({}): {} computers.", inputs_seen, computers.len());
            }
            instruction => {
                computers.par_iter_mut().for_each(|computer| {
                    computer.0.operation(instruction);
                });
            }
        }
    }
    lock.replace(computers.clone());
    computers
}

pub fn star_one(input: impl BufRead) -> usize {
    let computers = cached_run(input);
    computers
        .into_iter()
        .filter(|(computer, _x)| computer.memory[3] == 0)
        .max_by_key(|(_computer, input)| input.1)
        .unwrap()
        .1
         .1
}

pub fn star_two(input: impl BufRead) -> usize {
    let computers = cached_run(input);
    computers
        .into_iter()
        .filter(|(computer, _x)| computer.memory[3] == 0)
        .min_by_key(|(_computer, input)| input.0)
        .unwrap()
        .1
         .0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        let input = b"";
        assert_eq!(star_one(Cursor::new(input)), 35);
    }

    #[test]
    fn test_star_two() {
        let input = b"";
        assert_eq!(star_two(Cursor::new(input)), 3351);
    }
}
