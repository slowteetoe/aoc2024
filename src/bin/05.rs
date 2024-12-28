use std::{cmp::Ordering, collections::BTreeMap};

use itertools::Itertools;
use tracing::instrument;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rule_section, update_section) = input.split_once("\n\n").unwrap();
    let rules: Vec<(u32, u32)> = rule_section
        .lines()
        .map(|rule| {
            let (p1, p2) = rule.split_once("|").unwrap();
            (p1.parse::<u32>().unwrap(), p2.parse::<u32>().unwrap())
        })
        .collect();

    // ugh, this is very naive... (and bad!!)
    let mut solution = 0;
    let updates = update_section.lines().map(|line| {
        let split: Vec<_> = line.split(",").collect();
        let stuff = split
            .iter()
            .enumerate()
            .map(|(idx, num)| (num.parse::<u32>().unwrap(), idx))
            .collect::<BTreeMap<_, _>>();
        (split[split.len() / 2].parse::<u32>().unwrap(), stuff)
    });
    updates.for_each(|(midvalue, pages)| {
        let mut valid = true;
        for _ in &pages {
            for (pre, after) in &rules {
                let idx1 = pages.get(pre);
                let idx2 = pages.get(after);
                match (idx1, idx2) {
                    (Some(pre), Some(after)) => {
                        // rule applies, make sure it's valid
                        if pre > after {
                            valid = false;
                        }
                    }
                    (_, _) => continue, // doesn't apply
                }
            }
        }
        if valid {
            solution += midvalue;
        }
    });

    Some(solution)
}

#[instrument(skip(input))]
pub fn part_two(input: &str) -> Option<u32> {
    // complete rewrite for part 2 because my solution for 1 sucked.
    // we can turn the ordering rules (a|b) into a comparator and just sort
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
