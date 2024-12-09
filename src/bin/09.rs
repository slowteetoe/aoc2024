advent_of_code::solution!(9);

// First day this year that got me in that the example passes fine, but only because it stops at '9' and doesn't have to deal with 2+ digit numbers
// I figured this naive approach of generating the strings would be really slow and probably wouldn't work for part 2 anyhow

// TODO think more

fn decompress(input: &str) -> String {
    let mut id = 0;
    input
        .trim()
        .chars()
        .enumerate()
        .fold(String::new(), |mut acc, (idx, ch)| {
            let times = ch.to_digit(10).unwrap() as usize;
            if idx % 2 == 0 {
                acc.push_str(&id.to_string().repeat(times));
                id += 1;
            } else {
                acc.push_str(&".".repeat(times));
            }
            acc
        })
}

fn condense(input: &str) -> String {
    let mut de = input.to_string();
    let l = de.len() - 1;
    de.clone()
        .chars()
        .into_iter()
        .rev()
        .enumerate()
        .for_each(|(idx, ch)| {
            if let Some(n) = de.find(".") {
                if n < l - idx {
                    de.replace_range(n..=n, &ch.to_string());
                    de.replace_range(l - idx..=l - idx, ".");
                }
            }
        });
    de
}

fn generate_checksum(input: &str) -> u64 {
    input.chars().enumerate().fold(0u64, |acc, (idx, c)| {
        if c == '.' {
            acc
        } else {
            acc + (c.to_digit(10).unwrap() * idx as u32) as u64
        }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let decompressed = decompress(input);
    // println!("decompressed: {decompressed}");
    let condensed = condense(&decompressed);
    // println!("condensed: {condensed}");
    let checksum = generate_checksum(&condensed);
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress() {
        let result = decompress("2333133121414131402");
        assert_eq!("00...111...2...333.44.5555.6666.777.888899", result);
    }

    #[test]
    fn test_condense() {
        let result = condense("00...111...2...333.44.5555.6666.777.888899");
        assert_eq!("0099811188827773336446555566..............", result);
    }

    #[test]
    fn test_checksum() {
        let result = generate_checksum("0099811188827773336446555566..............");
        assert_eq!(1928, result);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
