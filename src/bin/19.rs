use itertools::Itertools;
use memoize::memoize;
use tracing::{debug, instrument};

advent_of_code::solution!(19);

#[derive(Debug)]
struct Problem {
    patterns: Vec<String>,
    designs: Vec<String>,
}

fn parse_input(input: &str) -> Problem {
    let patterns = input
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .into_iter()
        .map(|p| p.to_owned())
        .collect_vec();
    let designs = input
        .lines()
        .skip(2)
        .map(|line| line.trim().to_owned())
        .collect_vec();
    Problem { patterns, designs }
}

#[memoize]
#[instrument]
fn possible_to_construct(design: String, patterns: Vec<String>) -> u32 {
    if design == "" {
        return 1;
    }
    let ret: u32 = patterns
        .iter()
        .filter(|p| design.starts_with(*p))
        .map(|p| {
            let smaller = design[p.len()..].to_owned();
            possible_to_construct(smaller, patterns.clone())
        })
        .sum();
    if ret > 0 {
        1
    } else {
        0
    }
}

#[instrument(skip(input))]
pub fn part_one(input: &str) -> Option<u32> {
    let Problem { patterns, designs } = parse_input(input);

    let valid = designs.iter().fold(0u32, |acc, d| {
        acc + possible_to_construct(d.clone(), patterns.clone())
    });
    Some(valid)
}

pub fn part_two(_input: &str) -> Option<u32> {
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
