use std::collections::BTreeMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = vec![];
    let mut right = vec![];
    input.lines().for_each(|line| {
        let mut cols = line.split_ascii_whitespace();
        left.push(cols.next().unwrap().parse::<u32>().unwrap());
        right.push(cols.next().unwrap().parse::<u32>().unwrap());
    });
    left.sort();
    right.sort();
    let mut run = 0;
    for i in 0..left.len() {
        run += left[i].abs_diff(right[i]);
    }
    Some(run)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = vec![];
    let mut right = BTreeMap::<u32, u32>::new();
    input.lines().for_each(|line| {
        let mut cols = line.split_ascii_whitespace();
        left.push(cols.next().unwrap().parse::<u32>().unwrap());
        let k = cols.next().unwrap().parse().unwrap();
        right.entry(k).and_modify(|v| *v += 1).or_insert(1);
    });
    let mut simularity = 0;
    for v in left {
        if let Some(count) = right.get(&(v as u32)) {
            simularity += v * count;
        }
    }
    Some(simularity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
