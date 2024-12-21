use std::collections::HashSet;

use glam::IVec2;
use pathfinding::prelude::dijkstra;
// use tracing::debug;

advent_of_code::solution!(16);

#[derive(Debug)]
struct Grid {
    start: IVec2,
    end: IVec2,
    walls: HashSet<IVec2>,
}

impl Grid {
    fn new() -> Self {
        Self {
            start: IVec2 { x: 0, y: 0 },
            end: IVec2 { x: 0, y: 0 },
            walls: HashSet::new(),
        }
    }
}

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .fold(Grid::new(), |acc, (y, line)| {
            line.chars().enumerate().fold(acc, |mut acc, (x, ch)| {
                let pos = IVec2::new(x as i32, y as i32);
                match ch {
                    '#' => {
                        acc.walls.insert(pos);
                        ()
                    }
                    'S' => acc.start = pos,
                    'E' => acc.end = pos,
                    '.' => (),
                    _ => unreachable!("shouldn't be any other symbols in the grid"),
                };
                acc
            })
        })
}

#[tracing::instrument(skip(input))]
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    // debug!(?grid);
    let result = dijkstra(
        &(grid.start, IVec2::X),
        |(p, dir)| {
            // can always change direction for 1000 pts
            let mut successors = vec![((*p, dir.perp()), 1000), ((*p, -dir.perp()), 1000)];
            // if there's no wall, we can continue on in the current direction
            let next_pos = p + dir;
            if !grid.walls.contains(&next_pos) {
                successors.push(((next_pos, *dir), 1))
            }
            successors
        },
        |(p, _)| *p == grid.end,
    )
    .expect("some path found");
    Some(result.1)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_small_example() {
        let result = part_one(
            r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
",
        );
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
