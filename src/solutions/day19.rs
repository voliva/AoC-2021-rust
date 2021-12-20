use itertools::Itertools;
use queues::{Buffer, IsQueue};

use super::Solver;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::ops::{Add, Sub};
use std::str::FromStr;

pub struct Problem;

/*
* => Si hi ha 12 nodes en comu, hi ha d'haver un node que tingui la mateixa distancia a 11 dels altres nodes
*/

const OVERLAP: usize = 6;

impl Solver for Problem {
    type Input = Vec<Vec<Position>>;
    type Output1 = usize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).skip(1);

        let mut result = Vec::new();
        let mut active = Vec::new();

        for l in lines {
            if l == "" {
                continue;
            }
            if l.starts_with("---") {
                result.push(active);
                active = Vec::new();
            } else {
                active.push(l.parse().unwrap());
            }
        }
        result.push(active);

        return result;
    }

    fn solve_first(&self, scanners: &Self::Input) -> Result<Self::Output1, String> {
        let matches = match_scanners(&scanners);

        let merged = merge_scanners(&scanners, &matches, 0);

        Ok(merged.len())
    }

    fn solve_second(&self, scanners: &Self::Input) -> Result<Self::Output2, String> {
        let matches = match_scanners(&scanners);

        let positions = get_scanner_positions(&scanners, &matches, 0);

        let result = positions
            .iter()
            .map(|p1| positions.iter().map(|p2| p1.distance(p2)).max().unwrap())
            .max()
            .unwrap();

        Ok(result)
    }
}

fn get_scanner_positions(
    scanners: &Vec<Vec<Position>>,
    matches: &HashMap<usize, Vec<(usize, MatchTransform)>>,
    i_scanner: usize,
) -> Vec<Position> {
    let mut result = vec![Position { x: 0, y: 0, z: 0 }];

    for (i_other, transform) in matches.get(&i_scanner).unwrap() {
        let other_scanners = get_scanner_positions(scanners, matches, *i_other);

        result.extend(other_scanners.iter().map(|b| transform.apply(b.clone())));
    }

    result
}

fn merge_scanners(
    scanners: &Vec<Vec<Position>>,
    matches: &HashMap<usize, Vec<(usize, MatchTransform)>>,
    i_scanner: usize,
) -> HashSet<Position> {
    let scanner = &scanners[i_scanner];

    let mut result = HashSet::from_iter(scanner.iter().map(|x| x.clone()));

    for (i_other, transform) in matches.get(&i_scanner).unwrap() {
        let other_beacons = merge_scanners(scanners, matches, *i_other);

        result.extend(other_beacons.iter().map(|b| transform.apply(b.clone())));
    }

    result
}

fn match_scanners(scanners: &Vec<Vec<Position>>) -> HashMap<usize, Vec<(usize, MatchTransform)>> {
    let distances = scanners.iter().map(|s| get_distances(s)).collect_vec();

    let mut to_match = Buffer::<usize>::new(distances.len());
    to_match.add(0).unwrap();
    let mut unmatched = HashSet::<usize>::from_iter(1..distances.len());
    let mut matches: HashMap<usize, Vec<(usize, MatchTransform)>> = HashMap::new();

    while let Ok(i) = to_match.remove() {
        let v = matches.entry(i).or_insert(Vec::new());

        for (i_match, transform) in find_matches(&distances, scanners, i, &unmatched) {
            to_match.add(i_match).unwrap();
            unmatched.remove(&i_match);

            v.push((i_match, transform));
        }
    }

    assert!(unmatched.len() == 0, "some scanners unmatched");

    matches
}

fn find_matches(
    distances: &Vec<Vec<Vec<isize>>>,
    scanners: &Vec<Vec<Position>>,
    base_i: usize,
    posible_matches: &HashSet<usize>,
) -> Vec<(usize, MatchTransform)> {
    let base_match = &distances[base_i];
    let base_beacons = &scanners[base_i];

    posible_matches
        .iter()
        .filter_map(|i| {
            let posible_match = &distances[*i];
            let posible_match_beacons = &scanners[*i];

            return scanners_match(
                base_match,
                posible_match,
                base_beacons,
                posible_match_beacons,
            )
            .map(|transform| (*i, transform));
        })
        .collect_vec()
}

fn scanners_match(
    distances_a: &Vec<Vec<isize>>,
    distances_b: &Vec<Vec<isize>>,
    beacons_a: &Vec<Position>,
    beacons_b: &Vec<Position>,
) -> Option<MatchTransform> {
    let mut overlapping_nodes = 0;
    let mut matching_beacons = Vec::new();

    for a in 0..distances_a.len() {
        let beacon_a = &beacons_a[a];
        let node_dist_a = &distances_a[a];
        for b in 0..distances_b.len() {
            let beacon_b = &beacons_b[b];
            let node_dist_b = &distances_b[b];
            if count_equal_elements(node_dist_a, node_dist_b) >= OVERLAP - 1 {
                matching_beacons.push((beacon_a, beacon_b));
                overlapping_nodes += 1;
            }
            if overlapping_nodes >= OVERLAP {
                return Some(get_transform(&matching_beacons));
            }
        }
    }

    None
}

fn count_equal_elements(a: &Vec<isize>, b: &Vec<isize>) -> usize {
    let mut b_items: HashMap<isize, usize> = HashMap::new();
    for v in b {
        let r = b_items.entry(*v).or_insert(0);
        *r += 1;
    }

    return a
        .iter()
        .filter(|v| {
            let r = b_items.entry(**v).or_insert(0);
            if *r > 0 {
                *r -= 1;
                return true;
            }
            false
        })
        .count();
}

fn get_distances(positions: &Vec<Position>) -> Vec<Vec<isize>> {
    positions
        .iter()
        .map(|p0| {
            positions
                .iter()
                .filter(|p1| p0 != *p1)
                .map(|p1| p0.distance(p1))
                .collect()
        })
        .collect()
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
    Z,
}

impl Position {
    fn distance(self: &Position, to: &Position) -> isize {
        (self.x - to.x).abs() + (self.y - to.y).abs() + (self.z - to.z).abs()
    }
    fn has_unique_coords(self: &Position) -> bool {
        let abs = self.abs();
        return abs.x != abs.y && abs.x != abs.z && abs.y != abs.z;
    }
    fn has_zero(self: &Position) -> bool {
        return self.x == 0 || self.y == 0 || self.z == 0;
    }
    fn neg(self: &Position) -> Position {
        Position {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
    fn abs(self: &Position) -> Position {
        Position {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
    fn get_axis(self: &Position, axis: &Axis) -> isize {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }
    fn set_axis(self: &mut Position, axis: &Axis, value: isize) {
        match axis {
            Axis::X => self.x = value,
            Axis::Y => self.y = value,
            Axis::Z => self.z = value,
        }
    }
    fn swap_axis(self: &Position, (first, second): &(Axis, Axis)) -> Position {
        let mut copy = self.clone();

        copy.set_axis(first, self.get_axis(second));
        copy.set_axis(second, self.get_axis(first));

        copy
    }
    fn flip_axis(self: &Position, axis: &Axis) -> Position {
        let mut copy = self.clone();

        copy.set_axis(axis, -self.get_axis(axis));

        copy
    }
}

impl core::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({}, {}, {})", self.x, self.y, self.z))

        // .debug_tuple("Position")
        //     .field(&self.x)
        //     .field(&self.y)
        //     .field(&self.z)
        //     .finish()
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl FromStr for Position {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split(",").collect_vec();

        Ok(Position {
            x: values[0].parse()?,
            y: values[1].parse()?,
            z: values[2].parse()?,
        })
    }
}

/*Position { x: 638, y: 469, z: -766 }
Position { x: -580, y: -672, z: -374 }

Position { x: -698, y: -711, z: -544 }
Position { x: 600, y: 664, z: -596 }

1. Move first scanner to origin
trans_a = Translation { x: -638, y: -469, z: 766 }

2. Move second scanner to origin
trans_b = Translation { x: 580, y: 672, z: 374 }

3. Apply transforms to second pair
Position { x: -698-638, y: -711-469, z: -544+766 }
Position { x: 600+580, y: 664+672, z: -596+374 }

=

Position { x: -1336, y: -1180, z: 222 }
Position { x: 1180, y: 1336, z: -222 }

4. Verify pair is valid to find transform: all x-y-z are unique on each position

5. Match axis changes, and apply on second beacon
Some('x','y')

Position { x: -1336, y: -1180, z: 222 }
Position { x: 1336, y: 1180, z: -222 }

6. Match flipped axis
['x','y','z']

7. transform definition:

trans_b = Translation { x: 580, y: 672, z: 374 } // Move to origin
swap_axis Some('x', 'y')
flip_axis ['x','y','z']
-trans_a = Translation { x: -(-638), y: -(-469), z: -(766) } // Move to relative to a
*/
#[derive(Debug)]
struct MatchTransform {
    origin: Position,
    swap_axis: Vec<(Axis, Axis)>,
    flip_axis: Vec<Axis>,
    relative: Position,
}

impl MatchTransform {
    fn apply(self: &MatchTransform, target: Position) -> Position {
        let mut target = target + self.origin.clone();
        for swap in &self.swap_axis {
            target = target.swap_axis(swap);
        }
        target = self
            .flip_axis
            .iter()
            .fold(target, |acc, axis| acc.flip_axis(axis));
        return target + self.relative.clone();
    }
}

fn get_transform(matching_beacons: &Vec<(&Position, &Position)>) -> MatchTransform {
    let first_pair = &matching_beacons[0];

    // println!("first_pair {:?}", first_pair);

    // 1. Move first scanner to origin
    let trans_a = first_pair.0.neg();

    // 2. Move second scanner to origin
    let trans_b = first_pair.1.neg();

    let result = (&matching_beacons[1..])
        .iter()
        .find_map(|pair| {
            // 3. Apply transforms to second pair
            let (first_beacon, second_beacon) = (
                pair.0.clone() + trans_a.clone(),
                pair.1.clone() + trans_b.clone(),
            );

            // 4. Verify pair is valid to find transform: all x-y-z are unique on each position
            // ASSUMPTION I cut a corner here by not checking the second beacon
            if first_beacon.has_zero() || !first_beacon.has_unique_coords() {
                return None;
            }
            // println!(
            //     "good pair {:?} {:?} {:?}",
            //     pair, first_beacon, second_beacon
            // );

            // 5. Match axis changes, and apply on second beacon
            let swap_axis = if first_beacon.x.abs() == second_beacon.x.abs() {
                if first_beacon.y.abs() != second_beacon.y.abs() {
                    vec![(Axis::Y, Axis::Z)]
                } else {
                    vec![]
                }
            } else {
                if first_beacon.y.abs() == second_beacon.y.abs() {
                    vec![(Axis::X, Axis::Z)]
                } else if first_beacon.z.abs() == second_beacon.z.abs() {
                    vec![(Axis::X, Axis::Y)]
                } else {
                    // All of them are different
                    if first_beacon.x.abs() == second_beacon.y.abs() {
                        vec![(Axis::X, Axis::Y), (Axis::Y, Axis::Z)]
                    } else {
                        vec![(Axis::X, Axis::Z), (Axis::Y, Axis::Z)]
                    }
                }
            };

            let mut swapped_second_beacon = second_beacon.clone();
            for swap in &swap_axis {
                swapped_second_beacon = swapped_second_beacon.swap_axis(swap);
            }

            // 6. Match flipped axis
            let mut flip_axis = Vec::new();
            if first_beacon.x != swapped_second_beacon.x {
                flip_axis.push(Axis::X);
            }
            if first_beacon.y != swapped_second_beacon.y {
                flip_axis.push(Axis::Y);
            }
            if first_beacon.z != swapped_second_beacon.z {
                flip_axis.push(Axis::Z);
            }

            Some(MatchTransform {
                origin: trans_b.clone(), // Move second scanner to origin
                swap_axis,
                flip_axis,
                relative: first_pair.0.clone(), // Move relative to first scanner
            })
        })
        .unwrap();

    result
}
