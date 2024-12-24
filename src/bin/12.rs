use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;
use tracing::instrument;

advent_of_code::solution!(12);

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    contents: HashMap<(i32, i32), char>,
}

fn parse_input(input: &str) -> Grid {
    let contents = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as i32, y as i32), ch))
        })
        .flatten()
        .collect();
    Grid {
        rows: input.lines().count(),
        cols: input.lines().next().unwrap().chars().count(),
        contents,
    }
}

#[instrument]
fn process_grid(grid: Grid) -> Vec<HashSet<(i32, i32)>> {
    let mut regions: Vec<HashSet<_>> = vec![];
    let mut seen = HashSet::<_>::new();
    for y in 0..grid.rows as i32 {
        for x in 0..grid.cols as i32 {
            let this_pos = (x, y);
            if seen.contains(&this_pos) {
                continue;
            }
            let mut queue: VecDeque<_> = VecDeque::new();
            let mut this_region = HashSet::new();
            queue.push_front(this_pos);
            while !queue.is_empty() {
                let curr = queue.pop_front().unwrap();
                let c = grid.contents.get(&curr).unwrap();
                this_region.insert(curr.clone());
                seen.insert(curr);
                for d in DIRECTIONS {
                    let next_pos = (curr.0 + d.x, curr.1 + d.y);
                    if seen.contains(&next_pos) {
                        continue;
                    }
                    if let Some(nchar) = grid.contents.get(&next_pos) {
                        if c == nchar {
                            queue.push_front(next_pos.clone());
                        }
                    }
                }
            }
            regions.push(this_region);
        }
    }
    regions
}

fn calculate_cost(regions: Vec<HashSet<(i32, i32)>>) -> i32 {
    regions
        .iter()
        .map(|region| region.len() as i32 * perimeter(&region))
        .sum()
}

fn calculate_cost_part2(regions: Vec<HashSet<(i32, i32)>>) -> i32 {
    regions
        .iter()
        .map(|region| region.len() as i32 * number_sides(&region))
        .sum()
}

/// insight: the number of sides a shape has is equal to the number of corners
/// we just have to figure out how to count corners...
fn number_sides(region: &HashSet<(i32, i32)>) -> i32 {
    region.iter().fold(0i32, |mut acc, (x, y)| {
        // TODO figure out a better way to do this, iterate rotating 90deg?
        // brute is good enough for now
        let up = region.contains(&(*x, y - 1));
        let up_right = region.contains(&(x + 1, y - 1));
        let right = region.contains(&(x + 1, *y));
        let down_right = region.contains(&(x + 1, y + 1));
        let down = region.contains(&(*x, y + 1));
        let down_left = region.contains(&(x - 1, y + 1));
        let left = region.contains(&(x - 1, *y));
        let up_left = region.contains(&(x - 1, y - 1));

        if up && right && !up_right {
            // inside corner
            acc += 1;
        } else if !up && !right {
            // outside corner
            acc += 1;
        }

        if right && down && !down_right {
            acc += 1;
        } else if !right && !down {
            acc += 1;
        }

        if down && left && !down_left {
            acc += 1;
        } else if !down && !left {
            acc += 1;
        }

        if left && up && !up_left {
            acc += 1;
        } else if !left && !up {
            acc += 1;
        }

        acc
    })
}

fn perimeter(region: &HashSet<(i32, i32)>) -> i32 {
    region.iter().fold(0i32, |mut acc, pos| {
        acc += 4;
        for d in DIRECTIONS {
            if region.contains(&((pos.0 + d.x, pos.1 + d.y))) {
                acc -= 1;
            }
        }
        acc
    })
}

#[instrument(skip(input))]
pub fn part_one(input: &str) -> Option<i32> {
    let grid = parse_input(input);

    let regions = process_grid(grid);

    let cost = calculate_cost(regions);
    Some(cost)
}

pub fn part_two(input: &str) -> Option<i32> {
    let grid = parse_input(input);

    let regions = process_grid(grid);

    let cost = calculate_cost_part2(regions);
    Some(cost)
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[traced_test]
    #[test]
    fn test_small_example() {
        let result = part_one(
            r#"AAAA
BBCD
BBCC
EEEC"#,
        );
        assert_eq!(result, Some(140))
    }

    #[traced_test]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
