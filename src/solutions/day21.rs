use itertools::Itertools;

use super::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = (u8, u8);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let vec = file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.split(": ").collect_vec()[1].to_string())
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .collect_vec();
        (vec[0], vec[1])
    }

    fn solve_first(&self, (p1, p2): &Self::Input) -> Result<Self::Output1, String> {
        let mut positions = [*p1 - 1, *p2 - 1];
        let mut scores: [usize; 2] = [0, 0];
        let mut next_roll = 0;
        let mut total_rolls = 0;
        let mut turn = 0;

        while scores[0] < 1000 && scores[1] < 1000 {
            for _ in 0..3 {
                positions[turn] = (positions[turn] + next_roll + 1) % 10;
                next_roll = (next_roll + 1) % 100;

                total_rolls += 1;
            }
            scores[turn] += (positions[turn] + 1) as usize;

            turn = (turn + 1) % 2;
        }

        if scores[0] < 1000 {
            Ok(total_rolls * scores[0])
        } else {
            Ok(total_rolls * scores[1])
        }
    }

    fn solve_second(&self, (p1, p2): &Self::Input) -> Result<Self::Output2, String> {
        let position = [*p1 - 1, *p2 - 1];
        let mut cache = HashMap::new();

        let r = simulate4(position, [0, 0], 0, &mut cache);

        Ok(r.into_iter().max().unwrap())
    }
}

const DICE: [(u8, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn simulate4(
    p: [u8; 2],
    s: [usize; 2],
    turn: u8,
    cache: &mut HashMap<([u8; 2], [usize; 2], u8), [usize; 2]>,
) -> [usize; 2] {
    if s[0] >= 21 {
        return [1, 0];
    }
    if s[1] >= 21 {
        return [0, 1];
    }
    let key = (p, s, turn);
    if cache.contains_key(&key) {
        return cache.get(&key).unwrap().clone();
    }

    let universes = if turn == 0 {
        DICE.map(|(d, u)| {
            simulate4(
                [(p[0] + d) % 10, p[1]],
                [s[0] + ((p[0] + d) % 10) as usize + 1, s[1]],
                (turn + 1) % 2,
                cache,
            )
            .map(|v| v * u)
        })
    } else {
        DICE.map(|(d, u)| {
            simulate4(
                [p[0], (p[1] + d) % 10],
                [s[0], s[1] + ((p[1] + d) % 10) as usize + 1],
                (turn + 1) % 2,
                cache,
            )
            .map(|v| v * u)
        })
    };

    let result = universes
        .iter()
        .fold([0, 0], |acc, v| [acc[0] + v[0], acc[1] + v[1]]);
    cache.insert(key, result);
    return result;
}
