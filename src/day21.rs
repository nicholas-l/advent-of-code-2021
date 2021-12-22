use std::{collections::HashMap, io::BufRead};

use itertools::Itertools;

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut data = String::new();
    let _res = input.read_to_string(&mut data);

    let (mut pos1, mut pos2) = {
        let (p1, p2) = data.split_once('\n').unwrap();
        (
            p1.split_once(": ").unwrap().1.parse::<usize>().unwrap(),
            p2.split_once(": ").unwrap().1.parse::<usize>().unwrap(),
        )
    };
    let mut dice = 1;
    let mut turn = true;

    let mut score1 = 0;
    let mut score2 = 0;

    pos1 -= 1;
    pos2 -= 1;

    while score1 < 1000 && score2 < 1000 {
        let value = dice + dice + 1 + dice + 2;
        if turn {
            pos1 += value;
            pos1 = pos1 % 10;
            score1 += pos1 + 1;
        } else {
            pos2 += value;
            pos2 %= 10;
            score2 += pos2 + 1;
        }
        turn = !turn;
        dice += 3;
    }
    score1.min(score2) * (dice - 1)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn move_position(&mut self, _distance: usize) {
        todo!()
    }
}

fn play(
    player1: Player,
    player2: Player,
    turn: bool,
    cache: &mut HashMap<(Player, Player, bool), (usize, usize)>,
) -> (usize, usize) {
    let max_score = 20;
    if player1.score > max_score {
        return (1, 0);
    } else if player2.score > max_score {
        return (0, 1);
    } else if let Some(winners) = cache.get(&(player1.clone(), player2.clone(), turn)) {
        return winners.clone();
    } else {
        let res = [1, 2, 3]
            .iter()
            .combinations_with_replacement(3)
            .map(|dice| dice.into_iter().sum::<usize>())
            .map(|value| {
                // println!("{} {} {:?}", player1.score, player2.score, dice);
                let mut player1 = player1.clone();
                let mut player2 = player2.clone();

                if turn {
                    player1.position = (player1.position + value - 1) % 10 + 1;
                    player1.score += player1.position;
                } else {
                    player2.position = (player2.position + value - 1) % 10 + 1;
                    player2.score += player2.position;
                }
                return play(player1, player2, !turn, cache);
            })
            .fold((0, 0), |acc, (p1, p2)| (acc.0 + p1, acc.1 + p2));
        cache.insert((player1, player2, turn), res);
        res
    }
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut data = String::new();
    let _res = input.read_to_string(&mut data);

    let (pos1, pos2) = {
        let (p1, p2) = data.split_once('\n').unwrap();
        (
            p1.split_once(": ").unwrap().1.parse::<usize>().unwrap(),
            p2.split_once(": ").unwrap().1.parse::<usize>().unwrap(),
        )
    };

    let player1 = Player {
        position: pos1,
        score: 0,
    };

    let player2 = Player {
        position: pos2,
        score: 0,
    };

    let mut cache = HashMap::new();

    let turn = true;

    let res = play(player1, player2, turn, &mut cache);

    res.0.max(res.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        let input = b"Player 1 starting position: 4
Player 2 starting position: 8";
        assert_eq!(star_one(Cursor::new(input)), 739785);
    }

    #[test]
    fn test_star_two() {
        let input = b"Player 1 starting position: 4
Player 2 starting position: 8";
        assert_eq!(star_two(Cursor::new(input)), 444356092776315);
    }
}
