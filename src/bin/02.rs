use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_ascii_whitespace()
            .into_iter()
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        // check for delta of 1..=3 before sorting to check for strictly ASC or DESC to save some sort operations
        // saved roughly 10µs on benchmarks
        if !nums
            .iter()
            .tuple_windows()
            .all(|(a, b)| a.abs_diff(*b) <= 3 && a.abs_diff(*b) >= 1)
        {
            continue;
        }

        // TODO come up with better way of determining if all are ASC or DESC, sorting is fast enough for these sizes
        let mut copy = vec![0; nums.len()];
        copy.copy_from_slice(&nums[..]);
        copy.sort();

        if nums != copy {
            // not one of asc or desc (don't know which) so flip to see if it's the other ordering
            copy.reverse();
            if nums != copy {
                // neither asc or desc ordering
                continue;
            }
        }
        safe += 1;
    }
    Some(safe)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
