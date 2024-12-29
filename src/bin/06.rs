use std::collections::BTreeSet;

use itertools::Itertools;
use tracing::instrument;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Debug, Clone)]
pub struct Guard {
    pos: (usize, usize),
    dir: Dir,
    visited: BTreeSet<(usize, usize, Dir)>,
}

#[derive(Debug)]
struct Grid {
    dim_x: usize,
    dim_y: usize,
    walls: BTreeSet<(usize, usize)>, // keep track of wall locations
}

fn build_grid(input: &str) -> (Guard, Grid) {
    let mut guard = None;
    let mut walls = BTreeSet::new();
    input.lines().enumerate().for_each(|(row_num, line)| {
        line.chars().enumerate().for_each(|(col_num, c)| {
            if c == '^' {
                guard = Some(Guard {
                    pos: (col_num, row_num),
                    dir: Dir::N,
                    visited: BTreeSet::new(),
                });
            }
            if c == '#' {
                walls.insert((col_num, row_num));
            }
        });
    });
    (
        guard.unwrap(),
        Grid {
            dim_x: input.lines().next().unwrap().len(),
            dim_y: input.lines().count(),
            walls,
        },
    )
}

impl Guard {
    fn step(&mut self, grid: &Grid) -> Result<Option<((usize, usize), Dir)>, ()> {
        // println!("at {:?}  trying to go {:?}", self.pos, self.dir);
        // check bounds
        if self.visited.contains(&(self.pos.0, self.pos.1, self.dir)) {
            // println!("loop detected!");
            return Err(());
        }
        let next = match self.dir {
            Dir::N => {
                if self.pos.1 == 0 {
                    return Ok(None);
                } else {
                    Some((self.pos.0, self.pos.1 - 1))
                }
            }
            Dir::S => {
                if self.pos.1 == grid.dim_y {
                    return Ok(None);
                } else {
                    Some((self.pos.0, self.pos.1 + 1))
                }
            }
            Dir::E => {
                if self.pos.0 == grid.dim_x {
                    return Ok(None);
                } else {
                    Some((self.pos.0 + 1, self.pos.1))
                }
            }
            Dir::W => {
                if self.pos.0 == 0 {
                    return Ok(None);
                } else {
                    Some((self.pos.0 - 1, self.pos.1))
                }
            }
        }
        .unwrap();
        // insert here so we don't have to subtract out the exit
        self.visited.insert((self.pos.0, self.pos.1, self.dir));

        if grid.walls.contains(&next) {
            // println!("hit a wall, turning");
            self.dir = self.dir.perp();
            return Ok(Some((next, self.dir)));
        }
        self.pos = next;
        Ok(Some((next, self.dir)))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut guard, grid) = build_grid(input);

    while let Ok(Some(_next)) = guard.step(&grid) {}

    let unique = guard
        .visited
        .iter()
        .map(|(x, y, _)| (x, y))
        .unique()
        .count();

    Some(unique as u32)
}

#[instrument(skip(input))]
pub fn part_two(input: &str) -> Option<u32> {
    let (guard, mut grid) = build_grid(input);

    // ugly, but now need to differentiate between a successful step (which will be Ok(Some(_))), a successful exit (which will be Ok(None)) and a loop (which will be Err())
    // run through once to get the valid locations
    let mut pristine = guard.clone();
    while let Ok(Some(_next)) = pristine.step(&grid) {}

    // We now know all the possible locations for a new wall, see if adding exactly one creates a loop
    // debug!(?pristine.visited);

    let loops = pristine
        .visited
        .iter()
        .map(|(x, y, _)| (x, y))
        .unique()
        .map(|(x, y)| {
            let new_wall = (x.clone(), y.clone());
            // debug!("Introducing wall at {:?}", new_wall);
            let mut loop_detected = false;
            let mut exited = false;
            let mut new_guard = guard.clone();
            grid.walls.insert(new_wall);
            while !loop_detected && !exited {
                match new_guard.step(&grid) {
                    Ok(Some(_)) => (),
                    Ok(None) => {
                        exited = true;
                    }
                    Err(_) => {
                        loop_detected = true;
                    }
                }
            }
            grid.walls.remove(&new_wall);
            loop_detected
        })
        .filter(|v| *v == true)
        .collect_vec();

    Some(loops.len() as u32)
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[traced_test]
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
