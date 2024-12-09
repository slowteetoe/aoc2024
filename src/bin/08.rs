use std::{
    borrow::BorrowMut,
    collections::{BTreeMap, BTreeSet},
};

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

    let mut overall: BTreeSet<_> = BTreeSet::new();
    for (_val, points) in antennas {
        // println!("looking at {} which has points: {:?}", val, points);
        for (a, b) in points.iter().tuple_combinations() {
            let (dx, dy) = (a.0.abs_diff(b.0) as isize, a.1.abs_diff(b.1) as isize);
            let dir = (a.0 - b.0, a.1 - b.1);
            let mut antinodes = vec![];
            match dir {
                (x, y) if x > 0 && y > 0 => {
                    // a down and right of b
                    antinodes.push((a.0 + dx, a.1 + dy));
                    antinodes.push((b.0 - dx, b.1 - dy));
                }
                (x, y) if x > 0 && y < 0 => {
                    // a up and right of b
                    antinodes.push((a.0 + dx, a.1 - dy));
                    antinodes.push((b.0 - dx, b.1 + dy));
                }
                (x, y) if x < 0 && y > 0 => {
                    // a down and left of b
                    antinodes.push((a.0 - dx, a.1 + dy));
                    antinodes.push((b.0 + dx, b.1 - dy));
                }
                (x, y) if x < 0 && y < 0 => {
                    // a up and left of b
                    antinodes.push((a.0 - dx, a.1 - dy));
                    antinodes.push((b.0 + dx, b.1 + dy));
                }
                _ => unreachable!(),
            }
            let mut valid_nodes: BTreeSet<Point> = antinodes
                .iter()
                .filter_map(|(x, y)| {
                    if *x >= 0 && *x < grid[0].len() as isize && *y >= 0 && *y < grid.len() as isize
                    {
                        Some((*x, *y))
                    } else {
                        None
                    }
                })
                .collect();
            // println!("valid antinodes: {:?}", &valid_nodes);
            overall.append(valid_nodes.borrow_mut());
        }
    }
    Some(overall.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
