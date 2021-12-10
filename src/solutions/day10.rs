use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader.lines().map(|x| x.unwrap()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let result = input
            .iter()
            .map(|line| corrupted_score(line))
            .filter_map(|v| v)
            .sum();

        Ok(result)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let result = input
            .iter()
            .map(|line| check(line))
            .map(|s| match s {
                Status::Incomplete(v) => v.iter().fold(0, |acc, c| {
                    let score = match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    };
                    acc * 5 + score
                }),
                _ => 0,
            })
            .filter(|v| *v > 0)
            .sorted()
            .collect_vec();

        Ok(result[result.len() / 2])
    }
}

fn corrupted_score(line: &str) -> Option<usize> {
    let result = line
        .chars()
        .scan(Vec::new(), |acc, c| {
            let result = match c {
                '(' | '[' | '{' | '<' => {
                    acc.push(c);
                    None
                }
                ')' => {
                    if let Some('(') = acc.last() {
                        acc.pop();
                        None
                    } else {
                        Some(c)
                    }
                }
                ']' => {
                    if let Some('[') = acc.last() {
                        acc.pop();
                        None
                    } else {
                        Some(c)
                    }
                }
                '}' => {
                    if let Some('{') = acc.last() {
                        acc.pop();
                        None
                    } else {
                        Some(c)
                    }
                }
                '>' => {
                    if let Some('<') = acc.last() {
                        acc.pop();
                        None
                    } else {
                        Some(c)
                    }
                }
                _ => unreachable!(),
            };
            Some(result)
        })
        .filter_map(|v| v)
        .take(1)
        .collect_vec();

    match result.get(0) {
        Some(')') => Some(3),
        Some(']') => Some(57),
        Some('}') => Some(1197),
        Some('>') => Some(25137),
        Some(_) => unreachable!(),
        None => None,
    }
}

enum Status {
    Complete,
    Incomplete(Vec<char>),
    Corrupt(char),
}
fn check(line: &str) -> Status {
    let mut stack = Vec::new();

    for c in line.chars() {
        let corrupt_char = match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
                None
            }
            ')' => {
                if let Some('(') = stack.last() {
                    stack.pop();
                    None
                } else {
                    Some(c)
                }
            }
            ']' => {
                if let Some('[') = stack.last() {
                    stack.pop();
                    None
                } else {
                    Some(c)
                }
            }
            '}' => {
                if let Some('{') = stack.last() {
                    stack.pop();
                    None
                } else {
                    Some(c)
                }
            }
            '>' => {
                if let Some('<') = stack.last() {
                    stack.pop();
                    None
                } else {
                    Some(c)
                }
            }
            _ => unreachable!(),
        };

        if let Some(v) = corrupt_char {
            return Status::Corrupt(v);
        }
    }

    if stack.len() == 0 {
        Status::Complete
    } else {
        stack.reverse();
        Status::Incomplete(stack)
    }
}
