use std::ops::Range;

use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<d1>[0-9]{1,})\,(?<d2>[0-9]{1,})\)").unwrap();
    Some(
        re.captures_iter(input)
            .map(|m| {
                m.name("d1").unwrap().as_str().parse::<u32>().unwrap()
                    * m.name("d2").unwrap().as_str().parse::<u32>().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<d1>[0-9]{1,})\,(?<d2>[0-9]{1,})\)").unwrap();
    // But now we also have to find the boundaries of do() don't() regions
    let dos = Regex::new(r"do\(\)").unwrap();
    let do_locs: Vec<Range<usize>> = dos.find_iter(input).map(|m| m.range()).collect();

    let donts = Regex::new(r"don't\(\)").unwrap();
    let dont_locs: Vec<Range<usize>> = donts.find_iter(input).map(|m| m.range()).collect();

    let ignore_ranges = build_ignores(&do_locs, &dont_locs);

    Some(
        re.captures_iter(input)
            .filter(|c| {
                // ignore any operations that happen inside a don't() region
                !ignore_ranges
                    .iter()
                    .any(|i| i.contains(&c.get(0).unwrap().start()))
            })
            .map(|m| {
                m.name("d1").unwrap().as_str().parse::<u32>().unwrap()
                    * m.name("d2").unwrap().as_str().parse::<u32>().unwrap()
            })
            .sum(),
    )
}

fn build_ignores(dos: &Vec<Range<usize>>, donts: &Vec<Range<usize>>) -> Vec<Range<usize>> {
    let mut ignores = Vec::new();
    donts.iter().for_each(|r| {
        // for each don't range, find the next do range
        let next = dos.iter().filter(|d| d.end > r.start).map(|r| r.end).min();
        if next.is_some() {
            ignores.push(r.start..next.unwrap());
        } else {
            ignores.push(r.start..usize::max_value())
        }
    });
    ignores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
