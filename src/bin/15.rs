use bmp::{
    consts::{BLACK, BLUE, RED, WHITE_SMOKE},
    Image,
};
use glam::IVec2;
use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Eq)]
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
    /// returns Some(current robot position) if able to move, None otherwise
    fn execute(&mut self, instruction: &Movement) -> Option<IVec2> {
        println!("Trying to move {:?} at {:?}", &instruction, self.robot);
        let robot_idx = self.to_index(&self.robot);
        match instruction {
            Movement::Up => {
                let up1 = self.xy_to_index(self.robot.x, self.robot.y - 1);
                if self.contents[up1] == Contents::EmptySpace {
                    self.contents[robot_idx] = Contents::EmptySpace;
                    self.contents[up1] = Contents::Robot;
                    self.robot.y -= 1;
                    println!("moved robot up 1 to {:?}", self.robot);
                    Some(self.robot)
                } else if self.contents[up1] == Contents::Box
                    // need bounds check to make sure we stay on the same row
                    && self.robot.y - 2  >= 0
                    && self.contents[self.xy_to_index(self.robot.x, self.robot.y - 2)] == Contents::EmptySpace
                {
                    let up2 = self.xy_to_index(self.robot.x, self.robot.y - 2);
                    self.contents[robot_idx] = Contents::EmptySpace;
                    self.contents[up1] = Contents::Robot;
                    self.contents[up2] = Contents::Box;
                    self.robot.y -= 1;
                    println!("moved robot (and box) up 1 to {:?}", self.robot);
                    Some(self.robot)
                } else {
                    None
                }
            }
            Movement::Right => {
                if self.contents[robot_idx + 1] == Contents::EmptySpace {
                    self.contents[robot_idx] = Contents::EmptySpace;
                    self.contents[robot_idx + 1] = Contents::Robot;
                    self.robot.x += 1;
                    println!("moved robot right 1 to {:?}", self.robot);
                    Some(self.robot)
                } else if self.contents[robot_idx + 1] == Contents::Box
                    // need bounds check to make sure we stay on the same row
                    && self.robot.x + 2 < self.dim.x
                    && self.contents[robot_idx + 2] == Contents::EmptySpace
                {
                    self.contents[robot_idx] = Contents::EmptySpace;
                    self.contents[robot_idx + 1] = Contents::Robot;
                    self.contents[robot_idx + 2] = Contents::Box;
                    self.robot.x += 1;
                    println!("moved robot (and box) right 1 to {:?}", self.robot);
                    Some(self.robot)
                } else {
                    None
                }
            }
            Movement::Down => {
                let down1 = self.xy_to_index(self.robot.x, self.robot.y + 1);
                if self.contents[down1] == Contents::EmptySpace {
                    self.contents[robot_idx] = Contents::EmptySpace;
                    self.contents[down1] = Contents::Robot;
                    self.robot.y += 1;
                    println!("moved robot down 1 to {:?}", self.robot);
                    Some(self.robot)
                } else if self.contents[down1] == Contents::Box
                    // need bounds check to make sure we stay on the same row
                    && self.robot.y + 2  < self.dim.y
                    && self.contents[self.xy_to_index(self.robot.x, self.robot.y + 2)] == Contents::EmptySpace
                {
                    let down2 = self.xy_to_index(self.robot.x, self.robot.y + 2);
                    self.contents[robot_idx] = Contents::EmptySpace;
                    self.contents[down1] = Contents::Robot;
                    self.contents[down2] = Contents::Box;
                    self.robot.y += 1;
                    println!("moved robot (and box) down 1 to {:?}", self.robot);
                    Some(self.robot)
                } else {
                    None
                }
            }
            Movement::Left => {
                if self.contents[robot_idx - 1] == Contents::EmptySpace {
                    self.contents[robot_idx] = Contents::EmptySpace;
                    self.contents[robot_idx - 1] = Contents::Robot;
                    self.robot.x -= 1;
                    println!("moved robot left 1 to {:?}", self.robot);
                    Some(self.robot)
                } else if self.contents[robot_idx - 1] == Contents::Box
                    // need bounds check to make sure we stay on the same row
                    && self.robot.x - 2 >= 0
                    && self.contents[robot_idx - 2] == Contents::EmptySpace
                {
                    self.contents[robot_idx] = Contents::EmptySpace;
                    self.contents[robot_idx - 1] = Contents::Robot;
                    self.contents[robot_idx - 2] = Contents::Box;
                    self.robot.x -= 1;
                    println!("moved robot (and box) left 1 to {:?}", self.robot);
                    Some(self.robot)
                } else {
                    None
                }
            }
        }
    }

    fn xy_to_index(&self, x: i32, y: i32) -> usize {
        (y as usize * self.dim.x as usize) + x as usize
    }

    fn to_index(&self, pos: &IVec2) -> usize {
        ((pos.y as usize * self.dim.x as usize) + pos.x as usize)
            .try_into()
            .unwrap()
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
    dbg!(&grid);
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
    instructions.iter().enumerate().for_each(|(idx, instr)| {
        if grid.execute(instr).is_some() {
            grid.create_image(&format!("{:04}", idx));
        }
    });
    grid.create_image("end");
    None
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
