use bmp::{
    consts::{BLACK, BLUE, RED, WHITE_SMOKE},
    Image,
};
use glam::UVec2;
use itertools::Itertools;
use std::collections::VecDeque;

advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Contents {
    EmptySpace,
    Wall,
    Robot,
    Box,
}

#[derive(Debug)]
struct Grid {
    dim: UVec2,
    contents: Vec<Contents>,
    robot: UVec2,
}

impl Grid {
    /// returns Some(current robot position) if able to move, None otherwise
    fn execute(&mut self, instruction: &Movement) -> Option<UVec2> {
        match instruction {
            Movement::Up => self.try_move_up(self.robot),
            Movement::Right => self.try_move_right(self.robot),
            Movement::Down => self.try_move_down(self.robot),
            Movement::Left => self.try_move_left(self.robot),
        }
    }

    fn try_move_up(&mut self, pos: UVec2) -> Option<UVec2> {
        let mut next_pos = pos.clone();
        next_pos.y -= 1;
        let mut next = self.xy_to_index(next_pos.x, next_pos.y);
        let mut queue = VecDeque::new();
        while next_pos.y > 0
            && self.contents[next] != Contents::Wall
            && self.contents[next] != Contents::EmptySpace
        {
            queue.push_front(self.contents[next]);
            next_pos.y -= 1;
            next = self.xy_to_index(next_pos.x, next_pos.y);
        }
        if next_pos.y < 1 || self.contents[next] == Contents::Wall {
            // ran out of room, nothing to do
            None
        } else {
            let delta = self.dim.x as usize; // one row vertically
                                             // since we have a spot to put stuff, unwind the stack
            while !queue.is_empty() {
                self.contents[next] = queue.pop_front().expect("missing content");
                next += delta;
            }
            // move the robot itself, since it was on a robot square it's now empty
            self.contents[next] = Contents::Robot;
            self.contents[next + delta] = Contents::EmptySpace;
            self.robot.y -= 1;
            Some(self.robot)
        }
    }
    fn try_move_down(&mut self, pos: UVec2) -> Option<UVec2> {
        let mut next_pos = pos.clone();
        next_pos.y += 1;
        let mut next = self.xy_to_index(next_pos.x, next_pos.y);
        let mut queue = VecDeque::new();
        while next_pos.y <= self.dim.y
            && self.contents[next] != Contents::Wall
            && self.contents[next] != Contents::EmptySpace
        {
            queue.push_front(self.contents[next]);
            next_pos.y += 1;
            next = self.xy_to_index(next_pos.x, next_pos.y);
        }
        if next_pos.y > self.dim.y || self.contents[next] == Contents::Wall {
            None
        } else {
            let delta = self.dim.x as usize; // one row vertically
                                             // since we have a spot to put stuff, unwind the stack
            while !queue.is_empty() {
                self.contents[next] = queue.pop_front().expect("missing content");
                next -= delta;
            }
            // move the robot itself, since it was on a robot square it's now empty
            self.contents[next] = Contents::Robot;
            self.contents[next - delta] = Contents::EmptySpace;
            self.robot.y += 1;
            Some(self.robot)
        }
    }

    fn try_move_left(&mut self, pos: UVec2) -> Option<UVec2> {
        let mut next_pos = pos.clone();
        next_pos.x -= 1;
        let mut next = self.xy_to_index(next_pos.x, next_pos.y);
        let mut queue = VecDeque::new();
        while next_pos.x > 0
            && self.contents[next] != Contents::Wall
            && self.contents[next] != Contents::EmptySpace
        {
            queue.push_front(self.contents[next]);
            next_pos.x -= 1;
            next = self.xy_to_index(next_pos.x, next_pos.y);
        }
        if next_pos.x < 1 || self.contents[next] == Contents::Wall {
            None
        } else {
            // since we have a spot to put stuff, unwind the stack
            while !queue.is_empty() {
                self.contents[next] = queue.pop_front().expect("missing content");
                next += 1;
            }
            // move the robot itself, since it was on a robot square it's now empty
            self.contents[next] = Contents::Robot;
            self.contents[next + 1] = Contents::EmptySpace;
            self.robot.x -= 1;
            Some(self.robot)
        }
    }

    fn try_move_right(&mut self, pos: UVec2) -> Option<UVec2> {
        let mut next_pos = pos.clone();
        next_pos.x += 1;
        let mut next = self.xy_to_index(next_pos.x, next_pos.y);
        let mut queue = VecDeque::new();
        while next_pos.x < self.dim.x
            && self.contents[next] != Contents::Wall
            && self.contents[next] != Contents::EmptySpace
        {
            queue.push_front(self.contents[next]);
            next_pos.x += 1;
            next = self.xy_to_index(next_pos.x, next_pos.y);
        }
        if next_pos.x >= self.dim.x || self.contents[next] == Contents::Wall {
            None
        } else {
            // since we have a spot to put stuff, unwind the stack
            while !queue.is_empty() {
                self.contents[next] = queue.pop_front().expect("missing content");
                next -= 1;
            }
            // move the robot itself, since it was on a robot square it's now empty
            self.contents[next] = Contents::Robot;
            self.contents[next - 1] = Contents::EmptySpace;
            self.robot.x += 1;
            Some(self.robot)
        }
    }

    fn xy_to_index(&self, x: u32, y: u32) -> usize {
        (y as usize * self.dim.x as usize) + x as usize
    }

    fn create_image(&self, img_name: &str) {
        let mut img: Image = Image::new(50, 50);
        if cfg!(test) {
            img = Image::new(10, 10);
        }

        for (x, y) in img.coordinates() {
            img.set_pixel(x, y, BLACK);
        }

        self.contents
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != Contents::EmptySpace)
            .for_each(|(idx, c)| {
                let y = (idx / self.dim.x as usize) as u32;
                let x = (idx % self.dim.x as usize) as u32;

                match c {
                    Contents::Wall => img.set_pixel(x, y, WHITE_SMOKE),
                    Contents::Robot => img.set_pixel(x, y, RED),
                    Contents::Box => img.set_pixel(x, y, BLUE),
                    _ => unreachable!(),
                }
            });

        let _ = img.save(format!("data/images/15-{img_name}.bmp"));
    }

    fn gps_score(&self) -> u32 {
        self.contents
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, contents)| {
                if *contents == Contents::Box {
                    let (y, x) = (idx as u32 / self.dim.x, idx as u32 % self.dim.x);
                    acc + (100 * y) + x
                } else {
                    acc
                }
            })
    }
}

#[derive(Debug, PartialEq)]
enum Movement {
    Up,
    Right,
    Down,
    Left,
}

fn parse_input(input: &str) -> (Grid, Vec<Movement>) {
    let (grid, instr) = input
        .split_once("\n\n")
        .expect("did not find grid/instructions");

    let mut parsed_grid = vec![];
    let mut grid_y = 0;
    grid.lines().for_each(|line| {
        grid_y += 1;
        line.chars().for_each(|c| match c {
            '#' => parsed_grid.push(Contents::Wall),
            '.' => parsed_grid.push(Contents::EmptySpace),
            'O' => parsed_grid.push(Contents::Box),
            '@' => parsed_grid.push(Contents::Robot),
            _ => unreachable!(),
        });
    });

    let dim = UVec2::new((parsed_grid.len() / grid_y) as u32, grid_y as u32);
    let robot_pos = parsed_grid
        .iter()
        .position(|c| *c == Contents::Robot)
        .and_then(|pos| {
            Some(UVec2::new(
                pos as u32 % dim.x as u32,
                pos as u32 / dim.y as u32,
            ))
        })
        .expect("couldn't find robot");

    let directions = instr
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Movement::Left),
            '^' => Some(Movement::Up),
            '>' => Some(Movement::Right),
            'v' => Some(Movement::Down),
            _ => None,
        })
        .collect_vec();

    (
        Grid {
            dim,
            contents: parsed_grid,
            robot: robot_pos,
        },
        directions,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, instructions) = parse_input(input);
    instructions.iter().enumerate().for_each(|(idx, instr)| {
        grid.execute(instr);
        if cfg!(test) {
            grid.create_image(&format!("{:04}", idx));
        }
    });
    Some(grid.gps_score())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let result = part_one(input);
        assert_eq!(result, Some(2028))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
