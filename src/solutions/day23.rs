use itertools::Itertools;
use pathfinding::dijkstra;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = [[char; 2]; 4];
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
            [0, 1].map(|i| lines[i][0]),
            [0, 1].map(|i| lines[i][1]),
            [0, 1].map(|i| lines[i][2]),
            [0, 1].map(|i| lines[i][3]),
        ]
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let start = Node {
            pods: input
                .into_iter()
                .map(|chars| chars.iter().map(|c| char_to_pod(*c)).collect())
                .collect(),
            hallway: [EMPTY; 11],
        };

        let (_, cost) = dijkstra(&start, adjacent, is_solved).unwrap();

        Ok(cost)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let extra = [['D', 'D'], ['C', 'B'], ['B', 'A'], ['A', 'C']];

        let start = Node {
            pods: input
                .into_iter()
                .enumerate()
                .map(|(index, chars)| {
                    vec![
                        char_to_pod(chars[0]),
                        char_to_pod(extra[index][0]),
                        char_to_pod(extra[index][1]),
                        char_to_pod(chars[1]),
                    ]
                })
                .collect(),
            hallway: [EMPTY; 11],
        };

        let (_, cost) = dijkstra(&start, adjacent, is_solved).unwrap();

        Ok(cost)
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
    pods: Vec<Vec<u8>>,
    hallway: [u8; 11],
}
fn is_solved(node: &Node) -> bool {
    node.pods.iter().enumerate().all(|(index, p)| {
        let i = index as u8;
        p.iter().all(|p| *p == i)
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
        .map(|(index, pod)| {
            let i = index as u8;
            pod.iter().all(|p| *p == EMPTY || *p == i)
        })
        .collect_vec();

    // Hallway amphipods
    let hallway_moves = node
        .hallway
        .iter()
        .enumerate()
        .filter(|(_, a)| **a != EMPTY && pods_ready[**a as usize])
        .filter_map(|(index, _)| move_to_room(node, index));
    result.extend(hallway_moves);

    let pod_moves = node
        .pods
        .iter()
        .enumerate()
        .filter(|(index, pod)| {
            let i = *index as u8;
            let has_intruder = pod.iter().any(|p| *p != EMPTY && *p != i);
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

    let last_empty = node.pods[target_pod]
        .iter()
        .enumerate()
        .rev()
        .skip_while(|(_, v)| **v != EMPTY)
        .take(1)
        .next()
        .map(|(index, _)| index)
        .unwrap();
    let pod_length = last_empty + 1;

    let steps = ((target_hw as isize) - (index as isize)).abs() as usize + pod_length;
    let cost = steps * ENERGY_PER_STEP[target_pod];

    let mut result = node.clone();
    result.hallway[index] = EMPTY;
    result.pods[target_pod][last_empty] = target_pod as u8;

    Some((result, cost))
}

fn move_to_hallway(node: &Node, index: usize) -> Vec<(Node, usize)> {
    let start = ENTRIES[index];

    let pod = &node.pods[index];
    let first_full = pod
        .iter()
        .enumerate()
        .skip_while(|(_, v)| **v == EMPTY)
        .take(1)
        .next()
        .map(|(index, _)| index)
        .unwrap();
    let distance_to_hw = first_full + 1;

    let right = (start..node.hallway.len())
        .take_while(|pos| node.hallway[*pos] == EMPTY)
        .filter(|pos| !ENTRIES.contains(pos))
        .map(|pos| {
            let steps = distance_to_hw + pos - start;
            let mut result = node.clone();
            let cost = steps * ENERGY_PER_STEP[pod[first_full] as usize];
            result.hallway[pos] = pod[first_full];
            result.pods[index][first_full] = EMPTY;
            (result, cost)
        });
    let left = (0..=start)
        .rev()
        .take_while(|pos| node.hallway[*pos] == EMPTY)
        .filter(|pos| !ENTRIES.contains(pos))
        .map(|pos| {
            let steps = distance_to_hw + start - pos;
            let mut result = node.clone();
            let cost = steps * ENERGY_PER_STEP[pod[first_full] as usize];
            result.hallway[pos] = pod[first_full];
            result.pods[index][first_full] = EMPTY;
            (result, cost)
        });

    right.chain(left).collect()
}
