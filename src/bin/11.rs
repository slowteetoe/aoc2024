use itertools::Itertools;

advent_of_code::solution!(11);

fn blink(stones: Vec<u32>) -> Vec<u32> {
    stones
        .iter()
        .flat_map(|stone| {
            let s = format!("{stone}");
            match *stone {
                0 => vec![1],
                _ if s.len() % 2 == 0 => {
                    let (l, r) = s.split_at(s.len() / 2);
                    vec![l.parse().unwrap(), r.parse().unwrap()]
                }
                n => vec![n * 2024],
            }
        })
        .collect_vec()
}

fn read_stones(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .map(|c| c.parse::<u32>().unwrap())
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = read_stones(input);
    for _ in 0..25 {
        stones = blink(stones);
    }
    Some(stones.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

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
