use itertools::Itertools;

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

pub struct Problem;

impl Solver for Problem {
    type Input = (HashSet<Coord>, HashSet<Coord>, Coord);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();
        let mut down = HashSet::new();
        let mut right = HashSet::new();
        let size = Coord {
            x: lines[0].len(),
            y: lines.len(),
        };
        for (y, l) in lines.into_iter().enumerate() {
            for (x, c) in l.char_indices() {
                match c {
                    'v' => {
                        down.insert(Coord { x, y });
                    }
                    '>' => {
                        right.insert(Coord { x, y });
                    }
                    _ => {}
                }
            }
        }

        (right, down, size)
    }

    fn solve_first(&self, (right, down, size): &Self::Input) -> Result<Self::Output1, String> {
        let mut moves = 0;
        let mut right = right.clone();
        let mut down = down.clone();
        loop {
            moves += 1;
            let (r, d, done) = move_cuc(right, down, size);
            print_cuc(&r, &d, size);
            if done {
                return Ok(moves);
            }
            right = r;
            down = d;
        }
    }

    fn solve_second(&self, _: &Self::Input) -> Result<Self::Output2, String> {
        Ok(0)
    }
}

fn move_cuc(
    right: HashSet<Coord>,
    down: HashSet<Coord>,
    size: &Coord,
) -> (HashSet<Coord>, HashSet<Coord>, bool) {
    let mut done = true;

    let mut new_right = HashSet::new();
    for cuc in &right {
        let dest = Coord {
            x: (cuc.x + 1) % size.x,
            y: cuc.y,
        };
        if right.contains(&dest) || down.contains(&dest) {
            new_right.insert(cuc.clone());
        } else {
            done = false;
            new_right.insert(dest);
        }
    }

    let mut new_down = HashSet::new();
    for cuc in &down {
        let dest = Coord {
            x: cuc.x,
            y: (cuc.y + 1) % size.y,
        };
        if new_right.contains(&dest) || down.contains(&dest) {
            new_down.insert(cuc.clone());
        } else {
            done = false;
            new_down.insert(dest);
        }
    }

    (new_right, new_down, done)
}

fn print_cuc(right: &HashSet<Coord>, down: &HashSet<Coord>, size: &Coord) {
    for y in (0..size.y) {
        let mut cs = vec![];
        for x in (0..size.x) {
            let coord = Coord { x, y };
            if right.contains(&coord) {
                cs.push('>');
            } else if down.contains(&coord) {
                cs.push('v');
            } else {
                cs.push('.');
            }
        }
        println!("{}", cs.into_iter().collect::<String>());
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Coord {
    x: usize,
    y: usize,
}
