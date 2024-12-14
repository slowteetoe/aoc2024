use itertools::Itertools;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .enumerate()
        .map(|(row, line)| line.chars().enumerate().map(|(col, ch)| ch).collect_vec())
        .collect_vec();
    dbg!(&grid);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let result = part_one(
            r#"AAAA
BBCD
BBCC
EEEC"#,
        );
        assert_eq!(result, Some(140))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
