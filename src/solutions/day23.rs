use itertools::Itertools;
use pathfinding::dijkstra;

use super::Solver;
use std::array;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = [(char, char); 4];
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                line.split("")
                    .filter(|c| *c != "" && *c != "#" && *c != "." && *c != " ")
                    .map(|c| c.chars().next().unwrap())
                    .collect_vec()
            })
            .filter(|x| x.len() > 0)
            .collect_vec();

        [
            (lines[0][0], lines[1][0]),
            (lines[0][1], lines[1][1]),
            (lines[0][2], lines[1][2]),
            (lines[0][3], lines[1][3]),
        ]
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        println!("{:?}", input);

        let start = Node {
            pods: input.map(|(a, b)| (char_to_pod(a), char_to_pod(b))),
            hallway: [
                EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
            ],
        };

        let (_, cost) = dijkstra(&start, adjacent, is_solved).unwrap();

        Ok(cost)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}

fn char_to_pod(c: char) -> u8 {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => unreachable!(),
    }
}

const EMPTY: u8 = 5;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Node {
    pods: [(u8, u8); 4],
    hallway: [u8; 11],
}
fn is_solved(node: &Node) -> bool {
    node.pods.iter().enumerate().all(|(index, p)| {
        let i = index as u8;
        p.0 == i && p.1 == i
    })
}

fn adjacent(node: &Node) -> Vec<(Node, usize)> {
    // -> Places 2,4,6,8 of the hallway are forbidden
    // -> Amphipods on the hallway can only move to their pod, only when their pod is ready
    let mut result = vec![];

    let pods_ready = node
        .pods
        .iter()
        .enumerate()
        .map(|(index, (a, b))| {
            let i = index as u8;
            (*a == EMPTY || *a == i) && (*b == EMPTY || *b == i)
        })
        .collect_vec();

    // Hallway amphipods
    let hallway_moves = node
        .hallway
        .iter()
        .enumerate()
        .filter(|(index, a)| **a != EMPTY && pods_ready[**a as usize])
        .filter_map(|(index, _)| move_to_room(node, index));
    result.extend(hallway_moves);

    let pod_moves = node
        .pods
        .iter()
        .enumerate()
        .filter(|(index, (a, b))| {
            let i = *index as u8;
            let has_intruder = (*a != EMPTY && *a != i) || (*b != EMPTY && *b != i);
            has_intruder
        })
        .flat_map(|(index, _)| move_to_hallway(node, index));
    result.extend(pod_moves);

    result
}

lazy_static::lazy_static! {
    static ref ENERGY_PER_STEP: [usize; 4] = [1,10,100,1000];
    static ref ENTRIES: [usize; 4] = [2,4,6,8];
}

// Pre: room ready to be filled in. Only returns None if the hallway is busy
fn move_to_room(node: &Node, index: usize) -> Option<(Node, usize)> {
    let target_pod = node.hallway[index] as usize;
    let target_hw = ENTRIES[target_pod];

    let start = if index > target_hw {
        index - 1
    } else {
        index + 1
    };
    let min = target_hw.min(start);
    let max = target_hw.max(start);

    if node.hallway[min..=max].iter().any(|c| *c != EMPTY) {
        return None;
    }

    let steps = ((target_hw as isize) - (index as isize)).abs() as usize
        + (if node.pods[target_pod].1 == EMPTY {
            2
        } else {
            1
        });
    let cost = steps * ENERGY_PER_STEP[target_pod];

    let mut result = node.clone();
    result.hallway[index] = EMPTY;
    if result.pods[target_pod].1 == EMPTY {
        result.pods[target_pod].1 = target_pod as u8;
    } else {
        result.pods[target_pod].0 = target_pod as u8;
    }

    Some((result, cost))
}

fn move_to_hallway(node: &Node, index: usize) -> Vec<(Node, usize)> {
    let start = ENTRIES[index];

    let pod = node.pods[index];

    let distance_to_hw = if pod.0 != EMPTY { 1 } else { 2 };

    let right = (start..node.hallway.len())
        .take_while(|pos| node.hallway[*pos] == EMPTY)
        .filter(|pos| !ENTRIES.contains(pos))
        .map(|pos| {
            let steps = distance_to_hw + pos - start;
            let mut result = node.clone();
            if pod.0 != EMPTY {
                let cost = steps * ENERGY_PER_STEP[pod.0 as usize];
                result.hallway[pos] = pod.0;
                result.pods[index].0 = EMPTY;
                (result, cost)
            } else {
                let cost = steps * ENERGY_PER_STEP[pod.1 as usize];
                result.hallway[pos] = pod.1;
                result.pods[index].1 = EMPTY;
                (result, cost)
            }
        });
    let left = (0..=start)
        .rev()
        .take_while(|pos| node.hallway[*pos] == EMPTY)
        .filter(|pos| !ENTRIES.contains(pos))
        .map(|pos| {
            let steps = distance_to_hw + start - pos;
            let mut result = node.clone();
            if pod.0 != EMPTY {
                let cost = steps * ENERGY_PER_STEP[pod.0 as usize];
                result.hallway[pos] = pod.0;
                result.pods[index].0 = EMPTY;
                (result, cost)
            } else {
                let cost = steps * ENERGY_PER_STEP[pod.1 as usize];
                result.hallway[pos] = pod.1;
                result.pods[index].1 = EMPTY;
                (result, cost)
            }
        });

    right.chain(left).collect()
}
