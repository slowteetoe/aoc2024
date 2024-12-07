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
                    found += 1;
                }
                // look left upward
                if r >= 3
                    && c >= 3
                    && map[r - 1][c - 1] == 'M'
                    && map[r - 2][c - 2] == 'A'
                    && map[r - 3][c - 3] == 'S'
                {
                    found += 1;
                }
                // look up
                if r >= 3 && map[r - 1][c] == 'M' && map[r - 2][c] == 'A' && map[r - 3][c] == 'S' {
                    found += 1;
                }
                // look right upward
                if r >= 3
                    && c < row.len() - 3
                    && map[r - 1][c + 1] == 'M'
                    && map[r - 2][c + 2] == 'A'
                    && map[r - 3][c + 3] == 'S'
                {
                    found += 1;
                }
                // look right
                if c < row.len() - 3
                    && map[r][c + 1] == 'M'
                    && map[r][c + 2] == 'A'
                    && map[r][c + 3] == 'S'
                {
                    found += 1;
                }
                // look right downward
                if r < map.len() - 3
                    && c < row.len() - 3
                    && map[r + 1][c + 1] == 'M'
                    && map[r + 2][c + 2] == 'A'
                    && map[r + 3][c + 3] == 'S'
                {
                    found += 1;
                }
                // look down
                if r < map.len() - 3
                    && map[r + 1][c] == 'M'
                    && map[r + 2][c] == 'A'
                    && map[r + 3][c] == 'S'
                {
                    found += 1;
                }
                // look left downward
                if r < map.len() - 3
                    && c >= 3
                    && map[r + 1][c - 1] == 'M'
                    && map[r + 2][c - 2] == 'A'
                    && map[r + 3][c - 3] == 'S'
                {
                    found += 1;
                }
            }
        });
    });
    Some(found)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().map(|c| c).collect())
        .collect();

    let mut found = 0;
    map.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, val)| {
            if *val == 'A' && r >= 1 && r < map.len() - 1 && c >= 1 && c < row.len() - 1 {
                // M.S
                // .A.
                // M.S
                if map[r - 1][c - 1] == 'M'
                    && map[r - 1][c + 1] == 'S'
                    && map[r + 1][c - 1] == 'M'
                    && map[r + 1][c + 1] == 'S'
                {
                    found += 1;
                }
                // S.S
                // .A.
                // M.M
                if map[r - 1][c - 1] == 'S'
                    && map[r - 1][c + 1] == 'S'
                    && map[r + 1][c - 1] == 'M'
                    && map[r + 1][c + 1] == 'M'
                {
                    found += 1;
                }
                // M.M
                // .A.
                // S.S
                if map[r - 1][c - 1] == 'M'
                    && map[r - 1][c + 1] == 'M'
                    && map[r + 1][c - 1] == 'S'
                    && map[r + 1][c + 1] == 'S'
                {
                    found += 1;
                }
                // S.M
                // .A.
                // S.M
                if map[r - 1][c - 1] == 'S'
                    && map[r - 1][c + 1] == 'M'
                    && map[r + 1][c - 1] == 'S'
                    && map[r + 1][c + 1] == 'M'
                {
                    found += 1;
                }
            }
        })
    });
    Some(found)
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
        assert_eq!(result, Some(9));
    }
}
