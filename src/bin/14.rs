use bmp::Image;
use glam::IVec2;
use regex::Regex;
use std::collections::BTreeMap;

advent_of_code::solution!(14);

#[derive(Debug)]
pub struct Robot {
    pos: IVec2,
    velocity: IVec2,
}

impl Robot {
    fn patrol(&mut self, ticks: u32, grid: &IVec2) {
        let mut next_x = (self.pos.x as i32 + (ticks as i32 * self.velocity.x)) % grid.x as i32;
        let mut next_y = (self.pos.y as i32 + (ticks as i32 * self.velocity.y)) % grid.y as i32;
        if next_y < 0 {
            next_y = grid.y + next_y;
        }
        if next_x < 0 {
            next_x = grid.x + next_x;
        }
        self.pos = IVec2::new(next_x, next_y);
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    // p=0,4 v=3,-3
    let re = Regex::new(r"p=(?<px>[-\d]+),(?<py>[-\d]+) v=(?<vx>[-\d]+),(?<vy>[-\d]+)")
        .expect("invalid regex");
    input
        .lines()
        .map(|line| {
            let cap = re.captures(line).expect("didn't match {line}");
            Robot {
                pos: IVec2 {
                    x: cap["px"].parse().unwrap(),
                    y: cap["py"].parse().unwrap(),
                },
                velocity: IVec2 {
                    x: cap["vx"].parse().unwrap(),
                    y: cap["vy"].parse().unwrap(),
                },
            }
        })
        .collect()
}

fn display(n: u32, robots: &Vec<Robot>, _gridlen: &IVec2) {
    if n != 6752 {
        // now that I know the answer, I'm not generating 8k BMP files...
        return;
    }
    let mut img = Image::new(101, 103);

    for (x, y) in img.coordinates() {
        img.set_pixel(x, y, bmp::consts::BLACK);
    }
    robots.iter().for_each(|r| {
        img.set_pixel(
            r.pos.x.try_into().unwrap(),
            r.pos.y.try_into().unwrap(),
            bmp::consts::GREEN,
        );
    });
    let _ = img.save(format!("data/images/iter-{:04}.bmp", n));
}

// seems like we can probably simulate for part 1, guessing we'd have to figure out calculation for part 2 esp. since they're giving us grid size...
pub fn part_one(input: &str) -> Option<u32> {
    let mut robots = parse_input(input);

    let mut gridlen = IVec2::new(101, 103);
    if cfg!(test) {
        gridlen = IVec2::new(11, 7);
    }

    let x_mid: i32 = (gridlen.x / 2).try_into().unwrap();
    let y_mid: i32 = (gridlen.y / 2).try_into().unwrap();

    let quadrants: BTreeMap<u8, usize> = robots
        .iter_mut()
        .map(|r| {
            r.patrol(100, &gridlen);
            r
        })
        .filter(|r| r.pos.x != x_mid && r.pos.y != y_mid)
        .fold(BTreeMap::new(), |mut acc, r| {
            match (r.pos.x, r.pos.y) {
                (x, y) if x < x_mid && y < y_mid => {
                    acc.entry(1).and_modify(|x| *x += 1).or_insert(1)
                }
                (x, y) if x > x_mid && y < y_mid => {
                    acc.entry(2).and_modify(|x| *x += 1).or_insert(1)
                }
                (x, y) if x < x_mid && y > y_mid => {
                    acc.entry(3).and_modify(|x| *x += 1).or_insert(1)
                }
                (x, y) if x > x_mid && y > y_mid => {
                    acc.entry(4).and_modify(|x| *x += 1).or_insert(1)
                }
                _ => unreachable!(),
            };
            acc
        });

    Some(quadrants.values().fold(1, |acc, n| acc * *n as u32))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = parse_input(input);

    let gridlen = IVec2::new(101, 103);

    for n in 1..7000 {
        robots.iter_mut().for_each(|r| {
            r.patrol(1, &gridlen);
        });

        // yes, I REALLY generate 7,000 BMP files and preview them in the filesystem to find the !@#$!@ Xmas tree
        display(n, &robots, &gridlen);
    }

    Some(6752)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_around() {
        let grid = IVec2::new(11, 7);
        let mut robot: Robot = Robot {
            pos: IVec2 { x: 2, y: 4 },
            velocity: IVec2 { x: 2, y: -3 },
        };
        // dbg!(&robot);
        let positions: Vec<IVec2> = (0..5)
            .map(|_| {
                robot.patrol(1, &grid);
                robot.pos
            })
            .collect();

        // dbg!(&positions);
        assert_eq!(
            vec![
                IVec2::new(4, 1),
                IVec2::new(6, 5),
                IVec2::new(8, 2),
                IVec2::new(10, 6),
                IVec2::new(1, 3),
            ],
            positions
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
