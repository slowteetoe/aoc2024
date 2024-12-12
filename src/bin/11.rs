use itertools::Itertools;
use memoize::memoize;

advent_of_code::solution!(11);

fn blink(stones: Vec<u64>) -> Vec<u64> {
    stones.iter().flat_map(|stone| handle(*stone)).collect_vec()
}

#[memoize]
fn handle(n: u64) -> Vec<u64> {
    // TODO figure out how to use math instead of strings
    let s = format!("{n}");
    if n == 0 {
        vec![1]
    } else if s.len() % 2 == 0 {
        let (l, r) = s.split_at(s.len() / 2);
        vec![l.parse().unwrap(), r.parse().unwrap()]
    } else {
        vec![n * 2024]
    }
}

fn read_stones(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|c| c.parse().unwrap())
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = read_stones(input);
    for _ in 0..25 {
        stones = blink(stones);
    }
    Some(stones.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_double_zero() {
        assert_eq!(0, "00".parse::<u32>().unwrap());
    }

    #[test]
    fn test_part_one_subset() {
        let mut stones = read_stones("125 17");
        for _ in 0..6 {
            stones = blink(stones);
        }
        assert_eq!(stones.len(), 22);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
