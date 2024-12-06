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

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe = 0;
    for line in input.lines() {
        let mut nums: Vec<i32> = line
            .split_ascii_whitespace()
            .into_iter()
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        // check for delta of 1..=3 before sorting to check for strictly ASC or DESC to save some sort operations
        // saved roughly 10µs on benchmarks
        let mut faults = 0;
        let mut i = 0;
        while i < nums.len() - 1 {
            let diff = nums[i].abs_diff(nums[i + 1]);
            if diff < 1 || diff > 3 {
                println!(
                    "error, i={} ({} and {}), current faults={}",
                    i,
                    nums[i],
                    nums[i + 1],
                    faults
                );
                if i < nums.len() - 1 {
                    // we can increment the faults and remove the current i+1 and then just continue
                    nums.remove(i + 1);
                    faults += 1;
                    continue;
                    // // if we're not on the last comparison, peek ahead and see if ignoring this number would be ok
                    // let next_diff = nums[i].abs_diff(nums[i + 2]);
                    // println!(
                    //     "comparing {} and {} results in diff of {}",
                    //     nums[i],
                    //     nums[i + 2],
                    //     next_diff
                    // );
                    // if next_diff < 1 || next_diff > 3 {
                    //     faults += 1; // this will effectively cause the report to be unsafe
                    //     i += 1;
                    // }
                }
                faults += 1; // allow one out of order
            }
            i += 1;
        }

        dbg!(&faults);

        // let reports = nums
        //     .iter()
        //     .tuple_windows()
        //     .map(|(a, b)| a.abs_diff(*b) <= 3 && a.abs_diff(*b) >= 1)
        //     .counts_by(|c| c);

        // let falses = reports.get(&false);
        // dbg!(&falses);
        // if falses.is_some_and(|f| *f > 1) {
        //     // allow 1 false before report as unsafe
        //     continue;
        // }

        if faults > 1 {
            continue;
        }

        // TODO come up with better way of determining if all are ASC or DESC, sorting is fast enough for these sizes
        let mut copy = vec![0; nums.len()];
        copy.copy_from_slice(&nums[..]);

        dbg!(&copy);
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
