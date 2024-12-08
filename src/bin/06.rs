use std::collections::BTreeSet;

advent_of_code::solution!(6);

#[derive(Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(Debug)]
pub struct Guard {
    pos: (usize, usize),
    dir: Dir,
    visited: BTreeSet<(usize, usize)>,
}

type Grid = Vec<Vec<char>>;

pub fn build_grid(input: &str) -> (Guard, Grid) {
    let mut guard = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(row_num, line)| {
            line.chars()
                .enumerate()
                .map(|(col_num, c)| {
                    if c == '^' {
                        guard = Some(Guard {
                            pos: (col_num, row_num),
                            dir: Dir::N,
                            visited: BTreeSet::new(),
                        });
                    }
                    c
                })
                .collect()
        })
        .collect();
    (guard.unwrap(), grid)
}

impl Guard {
    fn step(&mut self, grid: &Grid) -> Option<(usize, usize)> {
        let mut next = self.pos;
        // println!(
        //     "at grid[{}][{}],  trying to go {:?}",
        //     next.1, next.0, self.dir
        // );
        match self.dir {
            Dir::N => {
                if self.pos.1 == 0 {
                    // println!("Escaped to the N!!");
                    return None;
                } else if self.pos.1 > 0 {
                    // space to move in that direction
                    next = (self.pos.0, self.pos.1 - 1);
                    if grid[next.1][next.0] == '#' {
                        // blocked, change direction but don't update pos or visited
                        // println!("Blocked to the north, turning E");
                        self.dir = Dir::E;
                        return Some(next);
                    }
                    self.pos = next;
                    self.visited.insert(next);
                }
            }
            Dir::S => {
                if self.pos.1 == grid.len() - 1 {
                    // println!("Escaped to the S!!");
                    return None;
                } else if self.pos.1 < grid.len() {
                    // space to move in that direction
                    next = (self.pos.0, self.pos.1 + 1);
                    if grid[next.1][next.0] == '#' {
                        // blocked, change direction but don't update pos or visited
                        // println!("Blocked to the south, turning W");
                        self.dir = Dir::W;
                        return Some(next);
                    }
                    self.pos = next;
                    self.visited.insert(next);
                }
            }
            Dir::E => {
                if self.pos.0 == grid.len() - 1 {
                    // println!("Escaped to the E!!");
                    return None;
                } else if self.pos.0 < grid.len() {
                    // space to move in that direction
                    next = (self.pos.0 + 1, self.pos.1);
                    if grid[next.1][next.0] == '#' {
                        // blocked, change direction but don't update pos or visited
                        // println!("Blocked to the east, turning S");
                        self.dir = Dir::S;
                        return Some(next);
                    }
                    self.pos = next;
                    self.visited.insert(next);
                }
            }
            Dir::W => {
                if self.pos.0 == 0 {
                    // println!("Escaped to the W!!");
                    return None;
                } else if self.pos.0 > 0 {
                    // space to move in that direction
                    next = (self.pos.0 - 1, self.pos.1);
                    if grid[next.1][next.0] == '#' {
                        // blocked, change direction but don't update pos or visited
                        // println!("Blocked to the west, turning N");
                        self.dir = Dir::N;
                        return Some(next);
                    }
                    self.pos = next;
                    self.visited.insert(next);
                }
            }
        }
        Some(next)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut guard, grid) = build_grid(input);

    while let Some(_next) = guard.step(&grid) {
        // println!("went to {},{}", next.0, next.1);
    }

    Some(guard.visited.iter().len() as u32 + 1) // add in final step
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
