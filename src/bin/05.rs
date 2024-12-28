use std::cmp::Ordering;

use itertools::Itertools;
use tracing::instrument;

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<(&str, &str)>, Vec<Vec<&str>>) {
    let (rules, pages) = input.split_once("\n\n").expect("two parts");
    let rules = rules
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").expect("a | b");
            (a, b)
        })
        .collect_vec();
    let pages = pages
        .lines()
        .map(|line| line.split(",").collect_vec())
        .collect_vec();
    (rules, pages)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = parse_input(input);
    let mut sum = 0u32;
    for update in pages {
        let sorted: Vec<&str> = update
            .clone()
            .iter()
            .map(|s| *s)
            .sorted_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else if rules.contains(&(*b, *a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .collect();
        if update == sorted {
            sum += update
                .get(update.len() / 2)
                .expect("midpoint")
                .parse::<u32>()
                .expect("parse midpoint");
        }
    }
    Some(sum)
}

#[instrument(skip(input))]
pub fn part_two(input: &str) -> Option<u32> {
    // complete rewrite for part 2 because my solution for 1 sucked.
    // we can turn the ordering rules (a|b) into a comparator and just sort
    let (rules, pages) = parse_input(input);
    let mut sum = 0u32;
    for update in pages {
        let sorted: Vec<&str> = update
            .clone()
            .iter()
            .map(|s| *s)
            .sorted_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else if rules.contains(&(*b, *a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .collect();
        if update != sorted {
            sum += sorted
                .get(sorted.len() / 2)
                .expect("midpoint")
                .parse::<u32>()
                .expect("parse midpoint");
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[traced_test]
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
