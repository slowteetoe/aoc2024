use std::collections::BTreeMap;

use glam::IVec2;
use itertools::Itertools;
use memoize::memoize;
use pathfinding::prelude::astar;
use tracing::{debug, instrument};

advent_of_code::solution!(21);

#[derive(Debug)]
struct NumericKeypad {
    cur: char,
    keys: BTreeMap<char, IVec2>,
}

impl NumericKeypad {
    fn new() -> Self {
        let mut keys = BTreeMap::new();
        keys.insert('7', IVec2::new(0, 0));
        keys.insert('8', IVec2::new(1, 0));
        keys.insert('9', IVec2::new(2, 0));
        keys.insert('4', IVec2::new(0, 1));
        keys.insert('5', IVec2::new(1, 1));
        keys.insert('6', IVec2::new(2, 1));
        keys.insert('1', IVec2::new(0, 2));
        keys.insert('2', IVec2::new(1, 2));
        keys.insert('3', IVec2::new(2, 2));
        keys.insert('0', IVec2::new(1, 3));
        keys.insert('A', IVec2::new(2, 3));
        Self { cur: 'A', keys }
    }

    /// take a character from 0..A and return the directional movements needed for that input
    fn translate(&mut self, c: char) -> Vec<char> {
        // tedious (10^10 entries?) but will work
        // match (self.pos, c) {
        //     ('A', 'A') => vec!['A'],
        //     ('A', '0') => vec!['<', 'A'],
        //     ('A', '3') => vec!['^', 'A'],
        //     ('A', '2') => vec!['^', '<', 'A'],
        //     _ => unreachable!(),
        // }
        todo!()
    }
}

/// yeah, this is probably not the best either...  maybe building up reachable paths from each char would be easier
#[memoize]
fn next_step_on_numeric_keypad(start: char, end: char) -> Vec<char> {
    let (path, _cost) = astar(
        &(start, 'X'),
        |p| match p.0 {
            'A' => vec![(('0', '<'), 1), (('3', '^'), 1)],
            '0' => vec![(('2', '^'), 1), (('A', '>'), 1)],
            '1' => vec![(('4', '^'), 1), (('2', '>'), 1)],
            '2' => vec![
                (('1', '<'), 1),
                (('3', '>'), 1),
                (('5', '^'), 1),
                (('0', 'v'), 1),
            ],
            '3' => vec![(('2', '<'), 1), (('6', '^'), 1), (('A', 'v'), 1)],
            '4' => vec![(('5', '>'), 1), (('7', '^'), 1)],
            '5' => vec![
                (('4', '<'), 1),
                (('6', '>'), 1),
                (('8', '^'), 1),
                (('2', 'v'), 1),
            ],
            '6' => vec![(('5', '<'), 1), (('9', '^'), 1), (('3', 'v'), 1)],
            '7' => vec![(('8', '>'), 1), (('4', 'v'), 1)],
            '8' => vec![(('7', '<'), 1), (('9', '>'), 1), (('5', 'v'), 1)],
            '9' => vec![(('8', '<'), 1), (('6', 'v'), 1)],
            'X' => vec![],
            _ => unreachable!(),
        },
        |_| 1,
        |p| p.0 == end,
    )
    .expect("should have been a path");
    path.iter().map(|n| n.1).collect()
}

// let's stick with our inefficient searching for now...
#[memoize]
fn next_step_on_directional_keypad(start: char, end: char) -> Vec<char> {
    let (path, _cost) = astar(
        &(start, 'X'),
        |p| match p.0 {
            'A' => vec![(('^', '<'), 1), (('>', 'v'), 1)],
            '<' => vec![(('v', '>'), 1)],
            '^' => vec![(('A', '>'), 1), (('v', 'v'), 1)],
            '>' => vec![(('v', '<'), 1), (('A', '^'), 1)],
            'v' => vec![(('<', '<'), 1), (('^', '^'), 1), (('>', '>'), 1)],
            'X' => vec![],
            _ => unreachable!(),
        },
        |_| 1,
        |p| p.0 == end,
    )
    .expect("should have been a path");
    path.iter().map(|n| n.1).collect()
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c).collect())
        .collect()
}

#[instrument(skip(input))]
pub fn part_one(input: &str) -> Option<u32> {
    let codes = parse(input);
    let numeric_movements = codes
        .iter()
        // .take(1)
        .map(|c| {
            let mut c = c.clone();
            c.insert(0, 'A');
            c
        })
        .map(|code| {
            let mut movements = vec![];
            code.iter().tuple_windows().for_each(|(start, end)| {
                let mut next = next_step_on_numeric_keypad(*start, *end);
                debug!("going from {start} to {end} is {:?}", next);
                next = next[1..].to_vec();
                next.push('A');
                movements.append(&mut next);
            });
            movements.iter().join("")
        })
        .collect_vec();
    // debug!(?numeric_movements);
    // translate the numeric_movements into movements on the directional keypad
    let directional_movements = numeric_movements
        .iter()
        .map(|m| {
            let m = "A".to_owned() + m.as_str();
            let mut movements = vec![];
            m.chars().tuple_windows().for_each(|(start, end)| {
                let mut next = next_step_on_directional_keypad(start, end);
                // wth?
                debug!("going from {end} to {start} is {:?}", next);
                next = next[1..].to_vec();
                next.push('A');
                movements.append(&mut next);
            });
            movements.iter().join("")
        })
        .collect_vec();
    // debug!(?directional_movements);
    // 3rd robot
    let third = directional_movements
        .iter()
        .map(|m| {
            let m = "A".to_owned() + m.as_str();
            let mut movements = vec![];
            m.chars().tuple_windows().for_each(|(start, end)| {
                let mut next = next_step_on_directional_keypad(start, end);
                // wth?
                println!("going from {end} to {start} is {:?}", next);
                next = next[1..].to_vec();
                next.push('A');
                movements.append(&mut next);
            });
            movements.iter().join("")
        })
        .collect_vec();
    // debug!(?third);
    let solutions = (0..third.iter().len())
        .map(|n| (codes[n].clone(), third[n].clone()))
        .collect::<BTreeMap<_, _>>();
    // debug!(?solutions);
    let complexity: u32 = solutions
        .iter()
        .map(|(code, seq)| {
            (
                code.iter()
                    .filter(|c| **c != 'A')
                    .join("")
                    .parse::<u32>()
                    .expect("should have been valid number"),
                seq.chars().count() as u32,
            )
        })
        .inspect(|(num, seq_len)| {
            println!("{} => {}", num, seq_len);
        })
        .map(|(num, seq_len)| num * seq_len)
        .sum();
    Some(complexity)
}

#[instrument(skip(input))]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[traced_test]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
