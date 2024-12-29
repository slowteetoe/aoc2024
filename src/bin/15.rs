use bmp::{
    consts::{BLACK, BLUE, RED, WHITE_SMOKE},
    Image,
};
use glam::IVec2;
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
    dim: IVec2,
    contents: Vec<Contents>,
    robot: IVec2,
}

impl Grid {
    fn try_move(&mut self, instruction: &Movement) -> Option<IVec2> {
        let mut next_pos = self.robot.clone() + instruction.delta();
        let mut next = self.pos_to_index(next_pos);
        let mut queue = VecDeque::new();
        while next_pos.y > 0
            && next_pos.x > 0
            && next_pos.y < self.dim.y
            && next_pos.x < self.dim.x
            && self.contents[next] != Contents::Wall
            && self.contents[next] != Contents::EmptySpace
        {
            queue.push_front(self.contents[next]);
            next_pos += instruction.delta();
            next = self.pos_to_index(next_pos);
        }
        if next_pos.y < 1
            || next_pos.x < 1
            || next_pos.y > self.dim.y
            || next_pos.x > self.dim.x
            || self.contents[next] == Contents::Wall
        {
            // ran out of space, don't move any boxes
            None
        } else {
            // unwind the queue, placing the boxes
            while !queue.is_empty() {
                self.contents[next] = queue.pop_front().expect("missing content");
                next_pos -= instruction.delta();
                next = self.pos_to_index(next_pos);
            }
            // move the robot to the new location, setting the original position to empty
            let original_robot_location = self.pos_to_index(self.robot);
            self.contents[original_robot_location] = Contents::EmptySpace;

            self.robot += instruction.delta();

            let new_robot_location = self.pos_to_index(self.robot);
            self.contents[new_robot_location] = Contents::Robot;

            Some(self.robot)
        }
    }

    fn pos_to_index(&self, pos: IVec2) -> usize {
        (pos.y as usize * self.dim.x as usize) + pos.x as usize
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
                    let (y, x) = (idx as i32 / self.dim.x, idx as i32 % self.dim.x);
                    acc + ((100 * y) + x) as u32
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

impl Movement {
    fn delta(&self) -> IVec2 {
        match self {
            Movement::Up => IVec2::NEG_Y,
            Movement::Right => IVec2::X,
            Movement::Down => IVec2::Y,
            Movement::Left => IVec2::NEG_X,
        }
    }
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

    let dim = IVec2::new((parsed_grid.len() / grid_y) as i32, grid_y as i32);
    let robot_pos = parsed_grid
        .iter()
        .position(|c| *c == Contents::Robot)
        .and_then(|pos| {
            Some(IVec2::new(
                pos as i32 % dim.x as i32,
                pos as i32 / dim.y as i32,
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
    instructions.iter().enumerate().for_each(|(_idx, instr)| {
        grid.try_move(instr);
        // if cfg!(test) {
        //     grid.create_image(&format!("{:04}", idx));
        // }
    });
    Some(grid.gps_score())
}

pub fn part_two(_input: &str) -> Option<u32> {
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
