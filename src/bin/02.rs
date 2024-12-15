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

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .into_iter()
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid(nums: &Vec<i32>) -> bool {
    let valid_diffs = nums
        .iter()
        .tuple_windows()
        .all(|(a, b)| a.abs_diff(*b) <= 3 && a.abs_diff(*b) > 0);
    if !valid_diffs {
        return false;
    }
    // unrolled a bit to avoid unnecessary clone()/sort()
    let mut sorted = nums.clone();
    sorted.sort();
    if nums == &sorted {
        return true;
    }
    sorted.reverse();
    nums == &sorted
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);

    let safe = reports
        .iter()
        .map(|report| {
            if is_valid(report) {
                1
            } else {
                let mut found_valid = false;
                // brute force, removing one element at a time - sucks, but works
                for n in 0..report.len() {
                    let mut candidate = report.clone();
                    candidate.remove(n);
                    if is_valid(&candidate) {
                        found_valid = true;
                        break;
                    }
                }
                if found_valid {
                    1
                } else {
                    0
                }
            }
        })
        .sum();
    Some(safe)
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
        assert_eq!(result, Some(4));
    }
}
