use std::collections::HashSet;

use glam::IVec2;
use itertools::Itertools;
use pathfinding::prelude::astar;
use tracing::{debug, instrument};

advent_of_code::solution!(20);

#[derive(Debug)]
struct Grid {
    dim: IVec2,
    start: IVec2,
    end: IVec2,
    walls: HashSet<IVec2>,
}

impl Grid {
    fn new() -> Self {
        Self {
            dim: IVec2::ZERO,
            start: IVec2::ZERO,
            end: IVec2::ZERO,
            walls: HashSet::new(),
        }
    }
}

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_Y, IVec2::Y, IVec2::NEG_X];

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .fold(Grid::new(), |mut acc, (y, line)| {
            if acc.dim.y < y as i32 {
                acc.dim.y = y as i32;
            }
            line.chars().enumerate().fold(acc, |mut acc, (x, ch)| {
                let pos = IVec2::new(x as i32, y as i32);
                if acc.dim.x < x as i32 {
                    acc.dim.x = x as i32;
                }
                match ch {
                    '#' => {
                        acc.walls.insert(pos);
                        ()
                    }
                    'S' => acc.start = pos,
                    'E' => acc.end = pos,
                    '.' => (),
                    _ => unreachable!("shouldn't be any other symbols in the grid"),
                };
                acc
            })
        })
}

#[instrument(skip(input))]
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    debug!(?grid);
    let (_path, no_cheat_cost) = astar(
        &grid.start,
        |p| {
            DIRECTIONS
                .iter()
                .filter_map(|d| {
                    let next = *p + d;
                    if !&grid.walls.contains(&next) {
                        Some((next, 1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        },
        |_| 1,
        |p| *p == grid.end,
    )
    .expect("original should have been solveable");

    // now we can skip up to 1 wall, since we have 2 picoseconds of cheat mode
    // we can also disregard outside walls when deciding which to skip
    let eligible_walls = &grid
        .walls
        .iter()
        .filter(|p|
            // it's not an outside wall
            p.x > 0 && p.x < grid.dim.x && p.y > 0 && p.y < grid.dim.y &&
            // there has to at least be one open space around it or our 2 picosecond cheat is pointless
            DIRECTIONS.iter().map(|d| *p + d).any(|new_pos| !grid.walls.contains(&new_pos)))
        .collect_vec();

    let target_savings = if cfg!(test) { 2 } else { 100 };

    let savings = eligible_walls
        .iter()
        .map(|cheat_pos| {
            let (_path, cheat_cost) = astar(
                &grid.start,
                |p| {
                    DIRECTIONS
                        .iter()
                        .filter_map(|d| {
                            let next = *p + d;
                            if !&grid.walls.contains(&next) || **cheat_pos == next {
                                Some((next, 1))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                },
                |_| 1,
                |p| *p == grid.end,
            )
            .expect("cheat should have been solveable");
            no_cheat_cost - cheat_cost
        })
        .filter(|savings| *savings >= target_savings)
        .count();

    Some(savings as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
