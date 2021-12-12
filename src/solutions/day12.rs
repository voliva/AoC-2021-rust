use super::Solver;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = (usize, usize, Vec<Vec<usize>>, HashSet<usize>);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.split("-").map(|x| x.to_owned()).collect_vec())
            .collect_vec();

        let mut index_to_cave = Vec::new();
        let mut cave_to_index = HashMap::new();
        let mut smalls = HashSet::new();
        for l in lines.iter() {
            let i = index_to_cave.len();
            if !cave_to_index.contains_key(&l[0]) {
                cave_to_index.insert(&l[0], i);
                if l[0] == l[0].to_lowercase() {
                    smalls.insert(i);
                }
                index_to_cave.push(&l[0]);
            }

            let i = index_to_cave.len();
            if !cave_to_index.contains_key(&l[1]) {
                cave_to_index.insert(&l[1], i);
                if l[1] == l[1].to_lowercase() {
                    smalls.insert(i);
                }
                index_to_cave.push(&l[1]);
            }
        }

        let mut graph = vec![vec![]; index_to_cave.len()];

        for l in lines.iter() {
            let i1 = cave_to_index.get(&l[0]).unwrap().to_owned();
            let i2 = cave_to_index.get(&l[1]).unwrap().to_owned();
            graph[i1].push(i2);
            graph[i2].push(i1);
        }

        let start = cave_to_index.get(&"start".to_string()).unwrap().to_owned();
        let end = cave_to_index.get(&"end".to_string()).unwrap().to_owned();
        (start, end, graph, smalls)
    }

    fn solve_first(
        &self,
        (start, end, graph, smalls): &Self::Input,
    ) -> Result<Self::Output1, String> {
        let mut paths_found = HashSet::new();
        find_paths(
            *start,
            *start,
            *end,
            &graph,
            &smalls,
            &mut HashSet::new(),
            false,
            "".to_string(),
            &mut paths_found,
        );
        Ok(paths_found.len())
    }

    fn solve_second(
        &self,
        (start, end, graph, smalls): &Self::Input,
    ) -> Result<Self::Output2, String> {
        let mut paths_found = HashSet::new();
        find_paths(
            *start,
            *start,
            *end,
            &graph,
            &smalls,
            &mut HashSet::new(),
            true,
            "".to_string(),
            &mut paths_found,
        );
        Ok(paths_found.len())
    }
}

fn find_paths(
    position: usize,
    start: usize,
    end: usize,
    graph: &Vec<Vec<usize>>,
    smalls: &HashSet<usize>,
    visited: &mut HashSet<usize>,
    wildcard: bool,
    path: String,
    paths_found: &mut HashSet<String>,
) {
    if position == end {
        paths_found.insert(path);
        return;
    }

    let edges_open = graph[position]
        .iter()
        .filter(|x| !visited.contains(x))
        .collect_vec();

    if edges_open.len() == 0 {
        return;
    }

    if smalls.contains(&position) {
        if wildcard && position != start {
            for e in edges_open.iter() {
                find_paths(
                    **e,
                    start,
                    end,
                    graph,
                    smalls,
                    visited,
                    false,
                    format!("{}{}", path, e),
                    paths_found,
                );
            }
        }

        visited.insert(position);
        for e in edges_open.iter() {
            find_paths(
                **e,
                start,
                end,
                graph,
                smalls,
                visited,
                wildcard,
                format!("{}{}", path, e),
                paths_found,
            );
        }
        visited.remove(&position);
    } else {
        for e in edges_open {
            find_paths(
                *e,
                start,
                end,
                graph,
                smalls,
                visited,
                wildcard,
                format!("{}{}", path, e),
                paths_found,
            );
        }
    }
}
