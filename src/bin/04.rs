advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().map(|c| c).collect())
        .collect();

    let mut found = 0;
    map.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, val)| {
            if *val == 'X' {
                // look left
                if c >= 3 && map[r][c - 1] == 'M' && map[r][c - 2] == 'A' && map[r][c - 3] == 'S' {
                    // println!("found looking left from {},{}", r, c);
                    found += 1;
                }
                // look left upward
                if r >= 3
                    && c >= 3
                    && map[r - 1][c - 1] == 'M'
                    && map[r - 2][c - 2] == 'A'
                    && map[r - 3][c - 3] == 'S'
                {
                    // println!("found looking left upward from {},{}", r, c);
                    found += 1;
                }
                // look up
                if r >= 3 && map[r - 1][c] == 'M' && map[r - 2][c] == 'A' && map[r - 3][c] == 'S' {
                    // println!("found looking upward from {},{}", r, c);
                    found += 1;
                }
                // look right upward
                if r >= 3
                    && c < row.len() - 3
                    && map[r - 1][c + 1] == 'M'
                    && map[r - 2][c + 2] == 'A'
                    && map[r - 3][c + 3] == 'S'
                {
                    // println!("found looking right upward from {},{}", r, c);
                    found += 1;
                }
                // look right
                if c < row.len() - 3
                    && map[r][c + 1] == 'M'
                    && map[r][c + 2] == 'A'
                    && map[r][c + 3] == 'S'
                {
                    // println!("found looking right from {},{}", r, c);
                    found += 1;
                }
                // look right downward
                if r < map.len() - 3
                    && c < row.len() - 3
                    && map[r + 1][c + 1] == 'M'
                    && map[r + 2][c + 2] == 'A'
                    && map[r + 3][c + 3] == 'S'
                {
                    // println!("found looking right downward from {},{}", r, c);
                    found += 1;
                }
                // look down
                if r < map.len() - 3
                    && map[r + 1][c] == 'M'
                    && map[r + 2][c] == 'A'
                    && map[r + 3][c] == 'S'
                {
                    // println!("found looking downward from {},{}", r, c);
                    found += 1;
                }
                // look left downward
                if r < map.len() - 3
                    && c >= 3
                    && map[r + 1][c - 1] == 'M'
                    && map[r + 2][c - 2] == 'A'
                    && map[r + 3][c - 3] == 'S'
                {
                    // println!("found looking left downward from {},{}", r, c);
                    found += 1;
                }
            }
        });
    });
    Some(found)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
