use itertools::Itertools;

use super::Solver;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Sub;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<([String; 10], [String; 4])>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                let (all_symbols, output) = line.split(" | ").collect_tuple().unwrap();

                let p_all_symbols: [String; 10] = parse_segments(all_symbols).try_into().unwrap();
                let p_output: [String; 4] = parse_segments(output).try_into().unwrap();
                (p_all_symbols, p_output)
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let result = input
            .into_iter()
            .map(|(_, x)| {
                x.iter().fold(0, |acc, s| match s.len() {
                    2 | 3 | 4 | 7 => acc + 1,
                    _ => acc,
                })
            })
            .sum();

        Ok(result)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(input.iter().map(|l| decode_line(l)).sum())
    }
}

fn parse_segments(str: &str) -> Vec<String> {
    let result: Vec<String> = str
        .split(" ")
        .map(|x| x.chars().sorted().collect())
        .collect();

    return result;
}

fn decode_line(line: &([String; 10], [String; 4])) -> usize {
    let (all_chars, out) = line;

    let mut chars_map = HashMap::new();
    chars_map.insert("012456", '0');
    chars_map.insert("25", '1');
    chars_map.insert("02346", '2');
    chars_map.insert("02356", '3');
    chars_map.insert("1235", '4');
    chars_map.insert("01356", '5');
    chars_map.insert("013456", '6');
    chars_map.insert("025", '7');
    chars_map.insert("0123456", '8');
    chars_map.insert("012356", '9');

    let wiring = get_wiring(all_chars);

    let result: usize = out
        .clone()
        .map(|display| {
            let segments: String = display
                .chars()
                .map(|c| char::from_digit(*wiring.get(&c).unwrap() as u32, 10).unwrap())
                .sorted()
                .collect();
            *(chars_map.get(segments.as_str()).unwrap())
        })
        .iter()
        .collect::<String>()
        .parse()
        .unwrap();

    result
}

fn get_wiring(all_chars: &[String; 10]) -> HashMap<char, usize> {
    let mut chars_used = HashSet::new();
    let mut result: [HashSet<char>; 7] =
        std::iter::repeat(HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']))
            .take(7)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

    let groups = group_by_length(all_chars);

    apply_known(&groups[1][0], 1, &mut chars_used, &mut result);
    apply_known(&groups[3][0], 4, &mut chars_used, &mut result);
    apply_known(&groups[2][0], 7, &mut chars_used, &mut result);

    // Now we can use the group of [0, 6] to get to know a bit more.
    // The middle segment currently has 2 options, the one we use for zero must keep at least one of them
    // Which means that some char of the middle segment must _not_ be on the code for 0.
    // Edit: Not exactly, that group also has 9.
    let zero_or_six_or_nine = &groups[5];
    let (zero, six_or_nine): (&str, [&str; 2]) = if result[3]
        .iter()
        .all(|c| zero_or_six_or_nine[0].contains(*c))
    {
        if result[3]
            .iter()
            .all(|c| zero_or_six_or_nine[1].contains(*c))
        {
            (
                &zero_or_six_or_nine[2],
                [&zero_or_six_or_nine[0], &zero_or_six_or_nine[1]],
            )
        } else {
            (
                &zero_or_six_or_nine[1],
                [&zero_or_six_or_nine[0], &zero_or_six_or_nine[2]],
            )
        }
    } else {
        (
            &zero_or_six_or_nine[0],
            [&zero_or_six_or_nine[1], &zero_or_six_or_nine[2]],
        )
    };
    apply_known(zero, 0, &mut chars_used, &mut result);

    let (six, nine) = if result[2].iter().all(|c| six_or_nine[0].contains(*c)) {
        (&six_or_nine[1], &six_or_nine[0])
    } else {
        (&six_or_nine[0], &six_or_nine[1])
    };

    apply_known(six, 6, &mut chars_used, &mut result);
    apply_known(nine, 9, &mut chars_used, &mut result);

    let mut ret = HashMap::new();
    for i in 0..result.len() {
        ret.insert(*(result[i].iter().next().unwrap()), i);
    }
    ret
}

fn group_by_length(all_chars: &[String; 10]) -> [Vec<String>; 7] {
    let mut result: [Vec<String>; 7] = std::iter::repeat(Vec::new())
        .take(7)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    for s in all_chars {
        result[s.len() - 1].push(s.to_string());
    }

    result
}

fn apply_known(
    display: &str,
    num: usize,
    chars_used: &mut HashSet<char>,
    result: &mut [HashSet<char>; 7],
) {
    let mut to_intersect: [HashSet<char>; 7] = std::iter::repeat(HashSet::new())
        .take(7)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut to_remove: [HashSet<char>; 7] = std::iter::repeat(HashSet::new())
        .take(7)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let segments = vec![
        vec![0, 1, 2, 4, 5, 6],
        vec![2, 5],
        vec![0, 2, 3, 4, 6], // 2
        vec![0, 2, 3, 5, 6], // 3
        vec![1, 3, 2, 5],
        vec![0, 1, 3, 5, 6], // 5
        vec![0, 1, 3, 4, 5, 6],
        vec![0, 2, 5],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 5, 6],
    ];

    apply_segments(
        &segments[num],
        &display,
        chars_used,
        &mut to_intersect,
        &mut to_remove,
    );

    // Assume result starts with full HashSet
    for i in 0..7 {
        if to_intersect[i].len() > 0 {
            result[i] = result[i]
                .intersection(&to_intersect[i])
                .map(|x| *x)
                .collect();
        }
    }
    for i in 0..7 {
        result[i] = result[i].sub(&to_remove[i]);
    }
}

fn apply_segments(
    segments: &Vec<usize>,
    display: &str,
    chars_used: &mut HashSet<char>,
    to_intersect: &mut [HashSet<char>; 7],
    to_remove: &mut [HashSet<char>; 7],
) {
    let mut rest_segments = HashSet::from([0, 1, 2, 3, 4, 5, 6]);

    for s in segments {
        rest_segments.remove(&s);
    }

    display.chars().for_each(|c| {
        for s in segments {
            to_intersect[*s].insert(c);
        }
    });

    display
        .chars()
        // .filter(|c| !chars_used.contains(c))
        .for_each(|c| {
            for s in &rest_segments {
                to_remove[*s].insert(c);
            }
        });

    chars_used.extend(display.chars());
}
