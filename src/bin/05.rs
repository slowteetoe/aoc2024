use std::collections::BTreeMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rule_section, update_section) = input.split_once("\n\n").unwrap();
    let rules: Vec<(u32, u32)> = rule_section
        .lines()
        .map(|rule| {
            let (p1, p2) = rule.split_once("|").unwrap();
            (p1.parse::<u32>().unwrap(), p2.parse::<u32>().unwrap())
        })
        .collect();

    // ugh, this is very naive... (and bad!!)
    let mut solution = 0;
    let updates = update_section.lines().map(|line| {
        let split: Vec<_> = line.split(",").collect();
        let stuff = split
            .iter()
            .enumerate()
            .map(|(idx, num)| (num.parse::<u32>().unwrap(), idx))
            .collect::<BTreeMap<_, _>>();
        (split[split.len() / 2].parse::<u32>().unwrap(), stuff)
    });
    updates.for_each(|(midvalue, pages)| {
        let mut valid = true;
        for _ in &pages {
            for (pre, after) in &rules {
                let idx1 = pages.get(pre);
                let idx2 = pages.get(after);
                match (idx1, idx2) {
                    (Some(pre), Some(after)) => {
                        // rule applies, make sure it's valid
                        if pre > after {
                            valid = false;
                        }
                    }
                    (_, _) => continue, // doesn't apply
                }
            }
        }
        if valid {
            solution += midvalue;
        }
    });

    Some(solution)
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
