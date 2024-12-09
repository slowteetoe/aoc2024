use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

advent_of_code::solution!(8);

type Point = (isize, isize);

fn parse_grid(input: &str) -> (Vec<Vec<char>>, BTreeMap<char, Vec<Point>>) {
    let mut antennas = BTreeMap::new();
    let grid = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, symbol)| {
                    match symbol {
                        '.' => {}
                        _ => {
                            antennas
                                .entry(symbol)
                                .and_modify(|m: &mut Vec<_>| m.push((col as isize, row as isize)))
                                .or_insert(vec![(col as isize, row as isize)]);
                        }
                    };
                    symbol
                })
                .collect_vec()
        })
        .collect_vec();
    (grid, antennas)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, antennas) = parse_grid(input);

    let overall: BTreeSet<_> = antennas
        .values()
        .map(|points| {
            points.iter().tuple_combinations().flat_map(|(a, b)| {
                let (dx, dy) = (a.0.abs_diff(b.0) as isize, a.1.abs_diff(b.1) as isize);
                let dir = (a.0 - b.0, a.1 - b.1);
                match dir {
                    (x, y) if x > 0 && y > 0 => {
                        // a down and right of b
                        vec![(a.0 + dx, a.1 + dy), (b.0 - dx, b.1 - dy)]
                    }
                    (x, y) if x > 0 && y < 0 => {
                        // a up and right of b
                        vec![(a.0 + dx, a.1 - dy), (b.0 - dx, b.1 + dy)]
                    }
                    (x, y) if x < 0 && y > 0 => {
                        // a down and left of b
                        vec![(a.0 - dx, a.1 + dy), (b.0 + dx, b.1 - dy)]
                    }
                    (x, y) if x < 0 && y < 0 => {
                        // a up and left of b
                        vec![(a.0 - dx, a.1 - dy), (b.0 + dx, b.1 + dy)]
                    }
                    _ => unreachable!(),
                }
            })
        })
        .flatten()
        .collect();

    let antinodes = overall
        .iter()
        .filter(|(x, y)| {
            // filter out ones that are outside the grid boundaries
            *x >= 0 && *x < grid[0].len() as isize && *y >= 0 && *y < grid.len() as isize
        })
        .count() as u32;

    Some(antinodes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, antennas) = parse_grid(input);

    let overall: BTreeSet<_> = antennas
        .values()
        .map(|points| {
            points.iter().tuple_combinations().flat_map(|(a, b)| {
                let (dx, dy) = (a.0.abs_diff(b.0) as isize, a.1.abs_diff(b.1) as isize);
                let dir = (a.0 - b.0, a.1 - b.1);
                // all the antennas are now antinodes too...
                let mut nodes = vec![*a, *b];
                let mut multiplier = 1;
                // this is pretty much brute force, going to generate a lot of useless points
                while multiplier * dx < grid[0].len() as isize
                    && multiplier * dx >= 0
                    && multiplier * dy >= 0
                    && multiplier * dy < grid.len() as isize
                {
                    match dir {
                        (x, y) if x > 0 && y > 0 => {
                            // a down and right of b
                            nodes.push((a.0 + multiplier * dx, a.1 + multiplier * dy));
                            nodes.push((b.0 - multiplier * dx, b.1 - multiplier * dy));
                        }
                        (x, y) if x > 0 && y < 0 => {
                            // a up and right of b
                            nodes.push((a.0 + multiplier * dx, a.1 - multiplier * dy));
                            nodes.push((b.0 - multiplier * dx, b.1 + multiplier * dy));
                        }
                        (x, y) if x < 0 && y > 0 => {
                            // a down and left of b
                            nodes.push((a.0 - multiplier * dx, a.1 + multiplier * dy));
                            nodes.push((b.0 + multiplier * dx, b.1 - multiplier * dy));
                        }
                        (x, y) if x < 0 && y < 0 => {
                            // a up and left of b
                            nodes.push((a.0 - multiplier * dx, a.1 - multiplier * dy));
                            nodes.push((b.0 + multiplier * dx, b.1 + multiplier * dy));
                        }
                        _ => unreachable!(),
                    }
                    multiplier += 1;
                }
                nodes
            })
        })
        .flatten()
        .collect();

    let antinodes = overall
        .iter()
        .filter(|(x, y)| {
            // filter out ones that are outside the grid boundaries
            *x >= 0 && *x < grid[0].len() as isize && *y >= 0 && *y < grid.len() as isize
        })
        .count() as u32;

    Some(antinodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
