use itertools::Itertools;
use memoize::memoize;

advent_of_code::solution!(11);

// This was for part 1, but count_stones works better
#[allow(unused)]
fn blink(stones: Vec<u64>) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|n| {
            if *n == 0 {
                vec![1]
            } else {
                // ilog10() - 10 is 1, 100 is 2, 1000 is 3
                // 123456.ilog10() = 5 + 1 = 6
                let num_digits = n.ilog10() + 1;
                // println!("looking at {n}, digits={num_digits}");
                if num_digits % 2 == 0 {
                    // 1000 = 10^3 (num digits / 2)
                    // 123456 / 1000 = 123, 123456 % 1000 = 456
                    vec![n / 10u64.pow(num_digits / 2), n % 10u64.pow(num_digits / 2)]
                } else {
                    vec![n * 2024]
                }
            }
        })
        .collect_vec()
}

#[memoize]
fn count_stones(stone: u64, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    let depth = depth - 1;
    if stone == 0 {
        return count_stones(1, depth);
    }
    let num_digits = stone.ilog10() + 1;
    if num_digits % 2 == 0 {
        return count_stones(stone / 10u64.pow(num_digits / 2), depth)
            + count_stones(stone % 10u64.pow(num_digits / 2), depth);
    } else {
        count_stones(stone * 2024, depth)
    }
}

fn read_stones(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|c| c.parse().unwrap())
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = read_stones(input);
    Some(stones.iter().map(|stone| count_stones(*stone, 25)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    // This has to be a counting or cycles problem, manipulating the vec is way too slow...
    let stones = read_stones(input);
    Some(stones.iter().map(|stone| count_stones(*stone, 75)).sum())
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
        // No examples for 75 blinks, double check the new algo works for 25
        assert_eq!(result, Some(55312));
    }
}
