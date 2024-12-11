use std::collections::BTreeSet;

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug)]
struct TrailSystem {
    trailheads: Vec<(usize, usize)>,
    trails: Vec<Vec<u32>>,
}

fn read_trails(input: &str) -> TrailSystem {
    let mut trailheads = vec![];
    let trails = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == '0' {
                        trailheads.push((col, row));
                    }
                    c.to_digit(10).unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    TrailSystem { trailheads, trails }
}

struct Hiker {}

impl Hiker {
    fn find_trails(&self, trailsystem: &TrailSystem) -> u32 {
        let trails = &trailsystem.trails;
        trailsystem
            .trailheads
            .iter()
            .map(|(x, y)| {
                let mut n = 1;
                let mut to_visit = BTreeSet::new();
                to_visit.insert((*x, *y));
                while n <= 9 {
                    let mut valid = BTreeSet::new();
                    for (col, row) in to_visit {
                        // north
                        if row > 0 && trails[row - 1][col] == n {
                            valid.insert((col, row - 1));
                        }
                        // east
                        if col < trails[0].len() - 1 && trails[row][col + 1] == n {
                            valid.insert(((col + 1), row));
                        }
                        // south
                        if row < trails.len() - 1 && trails[row + 1][col] == n {
                            valid.insert((col, (row + 1)));
                        }
                        // west
                        if col > 0 && trails[row][col - 1] == n {
                            valid.insert(((col - 1), row));
                        }
                    }
                    // dbg!(&n, &valid);
                    n += 1;

                    to_visit = valid;
                }
                to_visit.len() as u32
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let trailsystem = read_trails(input);
    // dbg!(&trailsystem);
    let hiker = Hiker {};
    let valid_trail_count = hiker.find_trails(&trailsystem);
    Some(valid_trail_count)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
