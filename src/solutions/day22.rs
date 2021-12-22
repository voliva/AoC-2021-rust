use super::Solver;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Command>;
    type Output1 = usize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut cuboids = HashSet::new();

        for command in input {
            for x in cap_range(&command.cuboid.x) {
                for y in cap_range(&command.cuboid.y) {
                    for z in cap_range(&command.cuboid.z) {
                        if command.on {
                            cuboids.insert((x, y, z));
                        } else {
                            cuboids.remove(&(x, y, z));
                        }
                    }
                }
            }
        }

        Ok(cuboids.len())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut space = HashSet::new();

        for command in input {
            if command.on {
                add_cuboid(&mut space, &command.cuboid);
            } else {
                remove_cuboid(&mut space, &command.cuboid);
            }
        }

        Ok(space
            .into_iter()
            .map(|(cuboid, on, _)| {
                let size = get_cuboid_size(&cuboid);
                if on {
                    size
                } else {
                    -size
                }
            })
            .sum())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Cuboid {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
    z: RangeInclusive<isize>,
}

impl Cuboid {
    fn contains(&self, cuboid: &Cuboid) -> bool {
        self.x.contains(cuboid.x.start())
            && self.x.contains(cuboid.x.end())
            && self.y.contains(cuboid.y.start())
            && self.y.contains(cuboid.y.end())
            && self.z.contains(cuboid.z.start())
            && self.z.contains(cuboid.z.end())
    }

    fn intersects(&self, cuboid: &Cuboid) -> Option<Cuboid> {
        let result = Cuboid {
            x: *(self.x.start().max(cuboid.x.start()))..=*(self.x.end().min(cuboid.x.end())),
            y: *(self.y.start().max(cuboid.y.start()))..=*(self.y.end().min(cuboid.y.end())),
            z: *(self.z.start().max(cuboid.z.start()))..=*(self.z.end().min(cuboid.z.end())),
        };
        if result.x.start() <= result.x.end()
            && result.y.start() <= result.y.end()
            && result.z.start() <= result.z.end()
        {
            Some(result)
        } else {
            None
        }
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn add_cuboid(space: &mut HashSet<(Cuboid, bool, usize)>, cuboid: &Cuboid) {
    let initial_space = space.clone();

    space.insert((cuboid.clone(), true, get_id()));
    for (c, on, id) in initial_space {
        if cuboid.contains(&c) {
            space.remove(&(c, on, id));
        } else if let Some(intersection) = cuboid.intersects(&c) {
            if on {
                space.insert((intersection, false, get_id()));
            } else {
                space.insert((intersection, true, get_id()));
            }
        }
    }
}

fn remove_cuboid(space: &mut HashSet<(Cuboid, bool, usize)>, cuboid: &Cuboid) {
    let initial_space = space.clone();

    for (c, on, id) in initial_space {
        if cuboid.contains(&c) {
            space.remove(&(c, on, id));
        } else if let Some(intersection) = cuboid.intersects(&c) {
            if on {
                space.insert((intersection, false, get_id()));
            } else {
                space.insert((intersection, true, get_id()));
            }
        }
    }
}

fn get_cuboid_size(cuboid: &Cuboid) -> isize {
    let x = cuboid.x.end() - cuboid.x.start() + 1;
    let y = cuboid.y.end() - cuboid.y.start() + 1;
    let z = cuboid.z.end() - cuboid.z.start() + 1;
    x * y * z
}

fn cap_range(range: &RangeInclusive<isize>) -> RangeInclusive<isize> {
    ((*range.start()).max(-50))..=((*range.end()).min(50))
}

#[derive(Debug)]
pub struct Command {
    on: bool,
    cuboid: Cuboid,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref LINE_RGX: Regex = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
        }

        let captures = LINE_RGX.captures(s).unwrap();

        let result = Command {
            on: captures[1].to_string() == "on",
            cuboid: Cuboid {
                x: captures[2].parse()?..=captures[3].parse()?,
                y: captures[4].parse()?..=captures[5].parse()?,
                z: captures[6].parse()?..=captures[7].parse()?,
            },
        };
        Ok(result)
    }
}
