use itertools::Itertools;

advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (target, nums) = line.split_once(":").unwrap();
            let nums = nums
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect_vec();
            (target.parse().unwrap(), nums)
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = parse_input(input);
    let valid = lines
        .iter()
        .map(|(target, nums)| {
            nums.iter()
                .fold(vec![], |mut acc, n| {
                    if acc.is_empty() {
                        acc.push(*n);
                        acc
                    } else {
                        acc.iter().flat_map(|a| generate(*a, *n)).collect_vec()
                    }
                })
                .into_iter()
                .filter(|num| *num == *target)
                .collect_vec()
        })
        .collect_vec();
    // dbg!(&valid);
    Some(
        valid
            .into_iter()
            .filter_map(|v| if !v.is_empty() { Some(v[0]) } else { None })
            .sum(),
    )
}

fn generate(a: u64, b: u64) -> Vec<u64> {
    vec![a * b, a + b]
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}