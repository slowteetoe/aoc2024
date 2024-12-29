use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    N = 0,
    S = 1,
    E = 2,
    W = 3,
}

impl Dir {
    // just because I like the name from Vec2
    fn perp(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N,
        }
    }
}

#[derive(Debug)]
pub struct Guard {
    pos: (usize, usize),
    dir: Dir,
    visited: BTreeSet<(usize, usize, Dir)>,
}

struct Grid {
    grid: BTreeMap<(usize, usize), char>,
    dim_x: usize,
    dim_y: usize,
}

fn build_grid(input: &str) -> (Guard, Grid) {
    let mut guard = None;
    let mut grid = BTreeMap::new();
    input.lines().enumerate().for_each(|(row_num, line)| {
        line.chars().enumerate().for_each(|(col_num, c)| {
            if c == '^' {
                guard = Some(Guard {
                    pos: (col_num, row_num),
                    dir: Dir::N,
                    visited: BTreeSet::new(),
                });
            }
            grid.insert((col_num, row_num), c);
        });
    });
    (
        guard.unwrap(),
        Grid {
            grid,
            dim_x: input.lines().next().unwrap().len(),
            dim_y: input.lines().count(),
        },
    )
}

impl Guard {
    fn step(&mut self, grid: &Grid) -> Option<((usize, usize), Dir)> {
        // println!("at {:?}  trying to go {:?}", self.pos, self.dir);
        // check bounds
        let next = match self.dir {
            Dir::N => {
                if self.pos.1 == 0 {
                    return None;
                } else {
                    Some((self.pos.0, self.pos.1 - 1))
                }
            }
            Dir::S => {
                if self.pos.1 == grid.dim_y {
                    return None;
                } else {
                    Some((self.pos.0, self.pos.1 + 1))
                }
            }
            Dir::E => {
                if self.pos.1 == grid.dim_x {
                    return None;
                } else {
                    Some((self.pos.0 + 1, self.pos.1))
                }
            }
            Dir::W => {
                if self.pos.1 == 0 {
                    return None;
                } else {
                    Some((self.pos.0 - 1, self.pos.1))
                }
            }
        }
        .unwrap();
        // insert here so we don't have to subtract out the exit
        self.visited.insert((self.pos.0, self.pos.1, self.dir));

        if grid.grid.get(&next) == Some(&'#') {
            self.dir = self.dir.perp();
            return Some((next, self.dir));
        }
        self.pos = next;
        Some((next, self.dir))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut guard, grid) = build_grid(input);

    while let Some(_next) = guard.step(&grid) {
        // println!("went to {},{}", next.0, next.1);
    }

    let unique = guard
        .visited
        .iter()
        .map(|(x, y, _)| (x, y))
        .unique()
        .count();

    // off by one somewhere - solution needs 5404 but gets 5403 unless I add in 1 (but example gets 42 if add in 1, and should be 41)
    Some(unique as u32)
}

// 16k empty spaces, so can't just randomly pick one... have to be smarter
// actually since the solution to part one was 5404, only that many to randomly choose from
// p1 kept track of locations visited, but not direction - since we're looking for loops, will need to take
// direction into account also I think...
pub fn part_two(input: &str) -> Option<u32> {
    let (mut guard, grid) = build_grid(input);

    while let Some(_next) = guard.step(&grid) {
        // println!("went to {},{}", next.0, next.1);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
