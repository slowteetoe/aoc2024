use glam::IVec2;
use itertools::Itertools;
use pathfinding::prelude::astar;
use tracing::{info, instrument};

advent_of_code::solution!(18);

#[derive(Debug)]
struct Grid {
    dim: usize,
    start: IVec2,
    end: IVec2,
    walls: Vec<IVec2>,
}

fn parse_input(input: &str) -> Grid {
    let walls = input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(",")
                .expect("should have been comma separated points");
            IVec2::new(
                x.parse().expect("x was invalid"),
                y.parse().expect("y was invalid"),
            )
        })
        .collect();
    let dim = if cfg!(test) { 6 } else { 70 };
    Grid {
        dim,
        start: IVec2::ZERO,
        end: IVec2::splat(dim as i32),
        walls,
    }
}

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

#[instrument(skip(input))]
pub fn part_one(input: &str) -> Option<u32> {
    let elapsed = if cfg!(test) { 12 } else { 1024 };
    let grid = parse_input(input);
    // tracing::debug!("{:?}", &grid.walls[..elapsed]);
    let solution = astar(
        &grid.start,
        |p| {
            DIRECTIONS
                .iter()
                .filter_map(|d| {
                    let next_pos = p + d;
                    if (0..=grid.dim).contains(&(next_pos.x as usize))
                        && (0..=grid.dim).contains(&(next_pos.y as usize))
                        && !grid.walls[..elapsed].contains(&next_pos)
                    {
                        Some((next_pos, 1))
                    } else {
                        None
                    }
                })
                .collect_vec()
        },
        |_| 0,
        |p| p == &grid.end,
    );
    Some(solution.expect("should have found a path").1)
}

// yeah, I don't feel like figuring this out right now... just brute force (binary search basically)
#[instrument(skip(input))]
pub fn part_two(input: &str) -> Option<String> {
    let known_good = if cfg!(test) { 12 } else { 1024 }; // we know there's a path at least through these because of part 1
    let input_len = if cfg!(test) { 25 } else { 3450 }; // length of the inputs
    let grid = parse_input(input);
    // tracing::debug!("{:?}", &grid.walls[..elapsed]);

    let first_blocked_idx = (known_good..=input_len)
        .collect_vec()
        .partition_point(|elapsed| {
            tracing::info!("checking path after {elapsed}");
            let solution = astar(
                &grid.start,
                |p| {
                    DIRECTIONS
                        .iter()
                        .filter_map(|d| {
                            let next_pos = p + d;
                            if (0..=grid.dim).contains(&(next_pos.x as usize))
                                && (0..=grid.dim).contains(&(next_pos.y as usize))
                                && !grid.walls[..*elapsed].contains(&next_pos)
                            {
                                Some((next_pos, 1))
                            } else {
                                None
                            }
                        })
                        .collect_vec()
                },
                |_| 0,
                |p| p == &grid.end,
            );
            solution.is_some()
        });
    info!(first_blocked_idx, known_good);

    Some(format!(
        "{},{}",
        grid.walls[first_blocked_idx + known_good - 1].x,
        grid.walls[first_blocked_idx + known_good - 1].y
    ))
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[traced_test]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[traced_test]
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_owned()));
    }
}
