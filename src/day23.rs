use std::{collections::HashMap, io::BufRead};

enum Position {
    Hallway,
    Room(char),
}

type AmphipodPositions = HashMap<(usize, usize), char>;


// Returns the possible locations to move to for a given amphipod.
fn get_posible_positions(
    map: &HashMap<(usize, usize), Position>,
    state: &AmphipodPositions,
    amp: (&(usize, usize), &char),
) -> Vec<((usize, usize), usize)> {
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let is_in_room = true; // fixme

    let mut stack = vec![*amp.0];
    let mut possible_positions = Vec::new();

    while let Some(pos) = stack.pop() {
        if pos
    }

}

fn get_possible_next_states<'a>(
    map: &'a HashMap<(usize, usize), Position>,
    state: &'a State,
) -> impl Iterator<Item = State> + 'a {
    state.positions.iter().flat_map(move |amp| {
        get_posible_positions(&map, &state.positions, amp)
            .iter()
            .map(|(new_pos, move_cost)| {
                let mut new_state = state.clone();
                new_state.positions.remove(&amp.0);
                let res = new_state.positions.insert(*new_pos, *amp.1);
                assert_eq!(res, None);
                new_state.cost = state.cost + move_cost;
                new_state
            })
            .collect::<Vec<_>>()
    })
}

// Should probably include a cost of that includes distances of amphipods to their room using cartesian distances

#[derive(Debug, Clone)]
struct State {
    positions: AmphipodPositions,
    cost: usize,
}

impl State {
    fn is_final(&self, map: &HashMap<(usize, usize), Position>) -> bool {
        self.positions
            .iter()
            .all(|(pos, value)| match map.get(pos) {
                Some(Position::Hallway) => false,
                Some(Position::Room(c)) => value == c,
                None => unreachable!(),
            })
    }
}

pub fn star_one(input: impl BufRead) -> usize {
    let a_chars = ['A', 'B', 'C', 'D'];
    // Maybe this should be a map and positions of amphipods.
    let (map, amphipods) = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let line = line.unwrap();
            let positions = line
                .chars()
                .enumerate()
                .filter(|(_j, c)| c != &'#' && c != &' ')
                .map(|(j, _c)| {
                    let value = if i == 1 {
                        Position::Hallway
                    } else {
                        Position::Room(a_chars[((j - 3) / 2)])
                    };
                    ((i, j), value)
                })
                .collect::<Vec<_>>();
            let amphipods = line
                .chars()
                .enumerate()
                .filter(|(_j, c)| c != &'#' && c != &'.')
                .map(|(j, c)| ((i, j), c))
                .collect::<Vec<_>>();
            (positions, amphipods)
        })
        .fold(
            (HashMap::new(), HashMap::new()),
            |(mut hm, mut all_amphipods), (positions, amphipods)| {
                hm.extend(positions);
                all_amphipods.extend(amphipods);
                (hm, all_amphipods)
            },
        );

    let mut stack = Vec::new();

    stack.push(State {
        positions: amphipods,
        cost: 0,
    });

    let mut best_state = None;

    while let Some(state) = stack.pop() {
        if best_state
            .as_ref()
            .map(|bs: &State| bs.cost > state.cost)
            .unwrap_or(true)
        {
            if state.is_final(&map) {
                best_state.replace(state);
            } else {
                for new_state in get_possible_next_states(&map, &state)
                    .filter(|p| best_state.as_ref().map(|bs| p.cost < bs.cost).unwrap_or(true))
                {
                }
            }
        }
    }

    0
}

pub fn star_two(input: impl BufRead) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        let input = b"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
        assert_eq!(star_one(Cursor::new(input)), 12521);
    }

    #[test]
    fn test_star_two() {
        let input = b"";
        assert_eq!(star_two(Cursor::new(input)), 3351);
    }
}
