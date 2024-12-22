use std::collections::BTreeSet;

use itertools::Itertools;
use tracing::{debug, instrument};

advent_of_code::solution!(19);

#[derive(Debug)]
struct Problem {
    patterns: Vec<String>,
    goals: Vec<String>,
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
    let goals = input
        .lines()
        .skip(2)
        .map(|line| line.trim().to_owned())
        .collect_vec();
    Problem { patterns, goals }
}

// generate all possible outputs from the available patterns, less than max_length
fn generate(patterns: &Vec<String>, max_length: usize) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    // ugh
    for k in 1..10 {
        patterns
            .iter()
            // combinations_with_replacement doesn't wrap back around, so this fails
            .combinations_with_replacement(k)
            // .inspect(|r| {
            //     debug!(?r);
            // })
            .map(|r| r.iter().join(""))
            .filter(|s| s.len() <= max_length)
            .for_each(|p| {
                result.insert(p);
            });
    }
    result
    // no luck understanding multi_cartesian_product
    // (0..10)
    //     .map(|_| patterns)
    //     .multi_cartesian_product()
    //     .flatten()
    //     .map(|s| s.to_owned())
    //     .collect::<BTreeSet<_>>()
}

/// Never a good sign when your part 1 takes > 1s... pretty sure that means this is a no-go
/// Gonna need a different approach
fn generate_permutations(s: &str, max_length: usize, patterns: &Vec<String>) -> Vec<String> {
    let mut result = vec![s.to_owned()];
    if s.len() >= max_length {
        return result;
    }
    for p in patterns {
        result.append(&mut generate_permutations(
            &(s.to_owned() + p),
            max_length,
            patterns,
        ));
    }
    result
}

#[instrument(skip(input))]
pub fn part_one(input: &str) -> Option<u32> {
    let problem = parse_input(input);
    let max_len = problem
        .goals
        .iter()
        .map(|p| p.len())
        .max()
        .expect("max length");

    // let buildable_designs = generate(&problem.patterns, max_len);
    let buildable_designs = generate_permutations("", max_len, &problem.patterns)
        .iter()
        .filter(|s| s.len() <= max_len)
        .map(|s| s.clone().to_owned())
        .collect::<BTreeSet<_>>();

    debug!(?problem, max_len, ?buildable_designs);

    let valid = problem
        .goals
        .iter()
        .filter(|g| buildable_designs.contains(g.as_str()))
        .count();
    Some(valid as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    // #[traced_test]
    // #[test]
    // fn test_generate() {
    //     let result = generate(
    //         &vec![
    //             String::from("a"),
    //             String::from("b"),
    //             String::from("c"),
    //             String::from("d"),
    //         ],
    //         6,
    //     );
    //     assert_eq!(BTreeSet::new(), result);
    // }

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
