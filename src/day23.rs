use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl From<char> for Amphipod {
    fn from(c: char) -> Self {
        match c {
            'A' => Amphipod::Amber,
            'B' => Amphipod::Bronze,
            'C' => Amphipod::Copper,
            'D' => Amphipod::Desert,
            _ => panic!("Unknown Amphipod: {}", c),
        }
    }
}

impl Amphipod {
    fn cost(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    fn is_room(&self, room: &char) -> bool {
        match self {
            Amphipod::Amber => room == &'A',
            Amphipod::Bronze => room == &'B',
            Amphipod::Copper => room == &'C',
            Amphipod::Desert => room == &'D',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Position {
    Hallway,
    Door,
    Room(char),
}

type Coordinate = (isize, isize);

type AmphipodPositions = BTreeMap<Coordinate, Amphipod>;

fn get_below(
    map: &HashMap<Coordinate, Position>,
    state: &AmphipodPositions,
    coordinate: &Coordinate,
) -> Vec<Amphipod> {
    let mut v = Vec::new();
    let mut i = 0;
    while let Some(Position::Room(_c)) = map.get(&(coordinate.0 + i, coordinate.1)) {
        let position = (coordinate.0 + i, coordinate.1);
        // Check if any of the other amphipods are in the room
        if let Some(amp) = state.get(&position) {
            v.push(*amp);
        }

        i += 1;
    }
    v
}

/// Returns the position of the best position to move into if the room is ready.
fn is_room_ready(
    map: &HashMap<Coordinate, Position>,
    state: &AmphipodPositions,
    coordinate: &Coordinate,
    amp: &Amphipod,
) -> Option<(Coordinate, usize)> {
    let mut i = 0;
    let mut best_position = None;
    assert_eq!(state.get(coordinate), None);
    while let Some(Position::Room(c)) = map.get(&(coordinate.0 + i, coordinate.1)) {
        let position = (coordinate.0 + i, coordinate.1);
        // Check if any of the other amphipods are in the room
        if let Some(amp2) = state.get(&position) {
            if amp != amp2 {
                return None;
            } else if best_position.is_none() {
                // If we have seen a position filled with the same amphipod, then set the previous position to the best position
                best_position.replace(((coordinate.0 + i - 1, coordinate.1), i as usize));
            }
        }

        if !amp.is_room(c) {
            return None;
        }

        i += 1;
    }

    // If we havent set the best position yet, then set it to the lowest point in the room.
    if best_position.is_none() {
        best_position.replace(((coordinate.0 + i - 1, coordinate.1), i as usize));
    }
    best_position
}

// Returns the possible locations to move to for a given amphipod.
fn get_posible_positions(
    map: &HashMap<Coordinate, Position>,
    state: &AmphipodPositions,
    (start_pos, amp): (&Coordinate, &Amphipod),
) -> Vec<(Coordinate, usize)> {
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let is_in_room = matches!(map.get(start_pos), Some(Position::Room(_)));

    if let Some(Position::Room(room)) = map.get(start_pos) {
        if amp.is_room(room)
            && get_below(map, state, start_pos)
                .iter()
                .all(|amp2| amp2 == amp)
        {
            return vec![];
        }
    }

    let mut stack = vec![(*start_pos, 0)];
    let mut possible_positions = Vec::new();
    let mut seen_positions = HashSet::new();

    while let Some((pos, cost)) = stack.pop() {
        // Get next free positions
        for dir in &dirs {
            let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if !seen_positions.contains(&next_pos) {
                seen_positions.insert(next_pos);
                let next_pos_state = state.get(&next_pos);
                if let Some(next_pos_value) = map.get(&next_pos) {
                    match (next_pos_value, next_pos_state) {
                        (Position::Hallway, None) => {
                            if is_in_room {
                                possible_positions.push((next_pos, cost + amp.cost()));
                            }
                            stack.push((next_pos, cost + amp.cost()));
                        }
                        (Position::Room(room), None) => {
                            if amp.is_room(room) {
                                if let Some((best_position, steps)) =
                                    is_room_ready(map, state, &next_pos, amp)
                                {
                                    return vec![(best_position, cost + steps * amp.cost())];
                                } else {
                                    stack.push((next_pos, cost + amp.cost()));
                                }
                            } else if is_in_room {
                                stack.push((next_pos, cost + amp.cost()));
                            }
                        }
                        (Position::Door, None) => {
                            stack.push((next_pos, cost + amp.cost()));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    possible_positions
}

// Should probably include a cost of that includes distances of amphipods to their room using cartesian distances

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    positions: AmphipodPositions,
    cost: usize,
}

impl State {
    fn is_final(&self, map: &HashMap<Coordinate, Position>) -> bool {
        self.positions.iter().all(|(pos, value)| {
            if let Some(Position::Room(c)) = map.get(pos) {
                value.is_room(c)
            } else {
                false
            }
        })
    }

    fn get_possible_next_states<'a>(
        &'a self,
        map: &'a HashMap<Coordinate, Position>,
    ) -> impl Iterator<Item = State> + 'a {
        self.positions.iter().flat_map(move |amp| {
            get_posible_positions(map, &self.positions, amp)
                .iter()
                .map(move |(new_pos, move_cost)| {
                    let mut new_state = self.clone();
                    new_state.positions.remove(amp.0);
                    let res = new_state.positions.insert(*new_pos, *amp.1);
                    assert_eq!(res, None);
                    assert!(move_cost > &0);
                    new_state.cost += move_cost;
                    new_state
                })
                .collect::<Vec<_>>()
        })
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // self.cost.cmp(&other.cost)
        other.cost.cmp(&self.cost)
    }
}

fn parse_room(
    lines: &[String],
) -> (
    HashMap<Coordinate, Position>,
    BTreeMap<Coordinate, Amphipod>,
) {
    let a_chars = ['A', 'B', 'C', 'D'];
    let doors = [3, 5, 7, 9];
    // Maybe this should be a map and positions of amphipods.
    let (map, amphipods) = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let positions = line
                .chars()
                .enumerate()
                .filter(|(_j, c)| c != &'#' && c != &' ')
                .map(|(j, _c)| {
                    let value = if i == 1 && doors.contains(&j) {
                        Position::Door
                    } else if i == 1 {
                        Position::Hallway
                    } else {
                        Position::Room(a_chars[((j - 3) / 2)])
                    };
                    ((i as isize, j as isize), value)
                })
                .collect::<Vec<_>>();
            let amphipods = line
                .chars()
                .enumerate()
                .filter(|(_j, c)| c != &'#' && c != &'.' && c != &' ')
                .map(|(j, c)| ((i as isize, j as isize), Amphipod::from(c)))
                .collect::<Vec<_>>();
            (positions, amphipods)
        })
        .fold(
            (HashMap::new(), BTreeMap::new()),
            |(mut hm, mut all_amphipods), (positions, amphipods)| {
                hm.extend(positions);
                all_amphipods.extend(amphipods);
                (hm, all_amphipods)
            },
        );
    (map, amphipods)
}

fn process(lines: &[String]) -> usize {
    let (map, amphipods) = parse_room(lines);

    let mut stack = BinaryHeap::new();

    stack.push(State {
        positions: amphipods,
        cost: 0,
    });

    let mut seen_states = HashSet::new();

    while let Some(state) = stack.pop() {
        if !seen_states.contains(&state) {
            seen_states.insert(state.clone());
            if state.is_final(&map) {
                return state.cost;
            } else {
                stack.extend(state.get_possible_next_states(&map));
            }
        }
    }
    panic!("No solution found");
}

pub fn star_one(input: impl BufRead) -> usize {
    let lines = input.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    process(&lines)
}

pub fn star_two(input: impl BufRead) -> usize {
    let mut lines = input.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    let mut extra = "  #D#C#B#A#
  #D#B#A#C#"
        .lines()
        .collect::<Vec<_>>();
    lines.insert(3, extra.pop().unwrap().to_string());
    lines.insert(3, extra.pop().unwrap().to_string());

    process(&lines)
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
    fn test_is_final() {
        let input = b"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";
        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };
        assert_eq!(state.is_final(&map), true);
    }

    #[test]
    fn test_4_is_final() {
        let input = b"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########";
        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };
        assert_eq!(state.is_final(&map), true);
    }

    #[test]
    fn test_get_positions() {
        let input = b"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";
        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        let positions = state.get_possible_next_states(&map).collect::<Vec<_>>();

        assert_eq!(positions.len(), 0);
        assert_eq!(positions, vec![]);
    }

    #[test]
    fn test_4_get_positions() {
        let input = b"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########";
        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        let positions = state.get_possible_next_states(&map).collect::<Vec<_>>();

        assert_eq!(positions.len(), 0);
        assert_eq!(positions, vec![]);
    }

    #[test]
    fn test_get_positions2() {
        let input = b"#############
#...........#
###B#A#C#D###
  #A#B#C#D#
  #########";
        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        let positions = state.get_possible_next_states(&map).collect::<Vec<_>>();

        assert_eq!(positions.len(), 14);
    }

    #[test]
    fn test_4_get_positions2() {
        let input = b"#############
#...........#
###B#A#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########";
        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        let positions = state.get_possible_next_states(&map).collect::<Vec<_>>();

        assert_eq!(positions.len(), 14);
    }

    #[test]
    fn test_get_positions3() {
        let input = b"#############
#...B.......#
###A#.#C#D###
  #A#B#C#D#
  #########";
        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        assert_eq!(map.get(&(2, 5)), Some(&Position::Room('B')));

        let positions = state.get_possible_next_states(&map).collect::<Vec<_>>();

        assert_eq!(positions.len(), 1);
    }

    #[test]
    fn test_4_get_positions3() {
        let input = b"#############
#...B.......#
###A#.#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########";
        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        assert_eq!(map.get(&(2, 5)), Some(&Position::Room('B')));

        let positions = state.get_possible_next_states(&map).collect::<Vec<_>>();

        assert_eq!(positions.len(), 1);
    }

    #[test]
    fn test_get_positions4() {
        let input = b"#############
#.B.B.......#
###A#.#C#D###
  #A#.#C#D#
  #########";

        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        assert_eq!(map.get(&(2, 5)), Some(&Position::Room('B')));

        let positions = state
            .positions
            .iter()
            .flat_map(|amp| get_posible_positions(&map, &state.positions, amp))
            .collect::<Vec<_>>();

        assert_eq!(positions.len(), 1);
        assert_eq!(positions, vec![((3, 5), 30)]);
    }

    #[test]
    fn test_4_get_positions4() {
        let input = b"#############
#.B.B.......#
###A#.#C#D###
  #A#.#C#D#
  #A#.#C#D#
  #A#.#C#D#
  #########";

        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        assert_eq!(map.get(&(2, 5)), Some(&Position::Room('B')));

        let positions = state
            .positions
            .iter()
            .flat_map(|amp| get_posible_positions(&map, &state.positions, amp))
            .collect::<Vec<_>>();

        assert_eq!(positions.len(), 1);
        assert_eq!(positions, vec![((5, 5), 50)]);
    }

    #[test]
    fn test_get_positions5() {
        let input = b"#############
#...........#
###A#.#C#D###
  #A#C#C#D#
  #########";
        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        assert_eq!(map.get(&(2, 5)), Some(&Position::Room('B')));

        let positions = state.get_possible_next_states(&map).collect::<Vec<_>>();

        assert_eq!(positions.len(), 7);
    }

    #[test]
    fn test_4_get_positions5() {
        let input = b"#############
#...........#
###A#.#C#D###
  #A#C#C#D#
  #A#C#C#D#
  #A#C#C#D#
  #########";
        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        assert_eq!(map.get(&(2, 5)), Some(&Position::Room('B')));

        let positions = state.get_possible_next_states(&map).collect::<Vec<_>>();

        assert_eq!(positions.len(), 7);
    }

    #[test]
    fn test_get_positions_in_hallway() {
        let input = b"#############
#.A.........#
###.#.#.#.###
  #.#.#.#.#
  #########";

        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        assert_eq!(map.get(&(2, 5)), Some(&Position::Room('B')));

        let positions = state
            .positions
            .iter()
            .flat_map(|amp| get_posible_positions(&map, &state.positions, amp))
            .collect::<Vec<_>>();

        assert_eq!(positions.len(), 1);
        assert_eq!(positions, vec![((3, 3), 3)]);
    }

    #[test]
    fn test_4_get_positions_in_hallway() {
        let input = b"#############
#.A.........#
###.#.#.#.###
  #.#.#.#.#
  #.#.#.#.#
  #.#.#.#.#
  #########";

        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        assert_eq!(map.get(&(2, 5)), Some(&Position::Room('B')));

        let positions = state
            .positions
            .iter()
            .flat_map(|amp| get_posible_positions(&map, &state.positions, amp))
            .collect::<Vec<_>>();

        assert_eq!(positions.len(), 1);
        assert_eq!(positions, vec![((5, 3), 5)]);
    }

    #[test]
    fn test_get_positions_in_hallway_with_partly_occupied() {
        let input = b"#############
#.A.........#
###.#.#.#.###
  #A#.#.#.#
  #########";

        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        let positions = state
            .positions
            .iter()
            .flat_map(|amp| get_posible_positions(&map, &state.positions, amp))
            .collect::<Vec<_>>();

        assert_eq!(positions, vec![((2, 3), 2)]);
    }

    #[test]
    fn test_4_get_positions_in_hallway_with_partly_occupied() {
        let input = b"#############
#.A.........#
###.#.#.#.###
  #A#.#.#.#
  #A#.#.#.#
  #A#.#.#.#
  #########";

        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        let positions = state
            .positions
            .iter()
            .flat_map(|amp| get_posible_positions(&map, &state.positions, amp))
            .collect::<Vec<_>>();

        assert_eq!(positions, vec![((2, 3), 2)]);
    }

    #[test]
    fn test_get_positions_in_hallway_other_occupied() {
        let input = b"#############
#.A.........#
###.#.#.#.###
  #B#.#.#.#
  #########";

        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        let positions = state
            .positions
            .iter()
            .flat_map(|amp| get_posible_positions(&map, &state.positions, amp))
            .collect::<Vec<_>>();

        assert_eq!(positions.len(), 1);
        // assert_eq!(positions, vec![]);
    }

    #[test]
    fn test_4_get_positions_in_hallway_other_occupied() {
        let input = b"#############
#.A.........#
###.#.#.#.###
  #B#.#.#.#
  #B#.#.#.#
  #B#.#.#.#
  #########";

        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        let positions = state
            .positions
            .iter()
            .flat_map(|amp| get_posible_positions(&map, &state.positions, amp))
            .collect::<Vec<_>>();

        assert_eq!(positions.len(), 1);
        // assert_eq!(positions, vec![]);
    }

    #[test]
    fn test_get_positions_other_occupied() {
        let input = b"#############
#...........#
###A#.#.#.###
  #B#.#.#.#
  #########";

        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        let positions = state
            .positions
            .iter()
            .flat_map(|amp| get_posible_positions(&map, &state.positions, amp))
            .collect::<Vec<_>>();

        assert_eq!(positions.len(), 7);
    }

    #[test]
    fn test_4_get_positions_other_occupied() {
        let input = b"#############
#...........#
###A#.#.#.###
  #B#.#.#.#
  #B#.#.#.#
  #B#.#.#.#
  #########";

        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        let positions = state
            .positions
            .iter()
            .flat_map(|amp| get_posible_positions(&map, &state.positions, amp))
            .collect::<Vec<_>>();

        assert_eq!(positions.len(), 7);
    }

    #[test]
    fn test_get_positions_above_occupied() {
        let input = b"#############
#...........#
###B#.#.#.###
  #A#.#.#.#
  #########";

        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        let positions = state
            .positions
            .iter()
            .flat_map(|amp| get_posible_positions(&map, &state.positions, amp))
            .collect::<Vec<_>>();

        assert_eq!(positions.len(), 1);
        // assert_eq!(positions, vec![((2, 5), 2)]);
    }

    #[test]
    fn test_4_get_positions_above_occupied() {
        let input = b"#############
#...........#
###B#.#.#.###
  #A#.#.#.#
  #A#.#.#.#
  #A#.#.#.#
  #########";

        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };

        let positions = state
            .positions
            .iter()
            .flat_map(|amp| get_posible_positions(&map, &state.positions, amp))
            .collect::<Vec<_>>();

        assert_eq!(positions.len(), 1);
        // assert_eq!(positions, vec![((2, 5), 2)]);
    }

    #[test]
    fn test_get_positions_final() {
        let input = b"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";
        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };
        let positions = state.get_possible_next_states(&map).collect::<Vec<_>>();

        assert_eq!(positions.len(), 0);
    }

    #[test]
    fn test_4_get_positions_final() {
        let input = b"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########";
        let lines = input.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let (map, amphipods) = parse_room(&lines);
        let state = State {
            positions: amphipods,
            cost: 0,
        };
        let positions = state.get_possible_next_states(&map).collect::<Vec<_>>();

        assert_eq!(positions.len(), 0);
    }
    #[test]
    fn test_star_two() {
        let input = b"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
        assert_eq!(star_two(Cursor::new(input)), 44169);
    }
}
