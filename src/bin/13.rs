use glam::U64Vec2;
use nalgebra::{Matrix2, Vector2};
use nalgebra_glm::determinant;
use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug, Default)]
struct ClawMachine {
    a: U64Vec2,
    b: U64Vec2,
    prize: U64Vec2,
}

// TODO - Probably would have been cleaner to break out Nom ...
fn parse_input(input: &str, prize_delta: u64) -> Vec<ClawMachine> {
    let button_regex = Regex::new(r"Button [A|B]: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
    input
        .split("\n\n")
        .map(|machine_def| {
            let [a_def, b_def, prize_def] = machine_def
                .split_terminator("\n")
                .take(3)
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            let a = button_regex
                .captures(a_def)
                .map(|cap| U64Vec2::new(cap["x"].parse().unwrap(), cap["y"].parse().unwrap()))
                .expect("wanted button a");
            let b = button_regex
                .captures(b_def)
                .map(|cap| U64Vec2::new(cap["x"].parse().unwrap(), cap["y"].parse().unwrap()))
                .expect("wanted button b");
            let prize = prize_regex
                .captures(prize_def)
                .map(|cap| {
                    U64Vec2::new(
                        cap["x"].parse::<u64>().unwrap() + prize_delta,
                        cap["y"].parse::<u64>().unwrap() + prize_delta,
                    )
                })
                .expect("wanted prize");
            ClawMachine { a, b, prize }
        })
        .collect()
}

/// return number of (A, B) buttton presses to win the prize
// let's be smart about this for once, looks like a linear algebra problem...
fn solve(machine: &ClawMachine) -> Option<(u64, u64)> {
    // use Cramer's rule to solve (https://doubtlet.com/calculator/cramers-rule/)

    // [ 94 22 ]
    // [ 34 67 ]
    let a = Matrix2::from_columns(&[
        Vector2::new(machine.a.x as f64, machine.b.x as f64),
        Vector2::new(machine.a.y as f64, machine.b.y as f64),
    ]);

    let det_a = determinant(&a);
    // dbg!(&det_a);

    // [ 8400 22 ]
    // [ 5400 67 ]

    let ax = Matrix2::from_columns(&[
        Vector2::new(machine.prize.x as f64, machine.prize.y as f64),
        Vector2::new(machine.b.x as f64, machine.b.y as f64),
    ]);

    let det_x = determinant(&ax) / det_a;
    // dbg!(&det_x);

    // [ 94 8400 ]
    // [ 34 5400 ]
    let ay = Matrix2::from_columns(&[
        Vector2::new(machine.a.x as f64, machine.a.y as f64),
        Vector2::new(machine.prize.x as f64, machine.prize.y as f64),
    ]);

    let det_y = determinant(&ay) / det_a;
    // dbg!(&det_y);

    // if number of button presses are integers, we've found a solution
    if det_x.fract() == 0.0 && det_y.fract() == 0.0 {
        Some((det_x as u64, det_y as u64))
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input, 0);

    Some(machines.iter().fold(0, |acc, m| {
        let solution = solve(m);
        if solution.is_some() {
            let (a, b) = solution.unwrap();
            acc + (a * 3 + b)
        } else {
            acc
        }
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let d = 10_000_000_000_000u64;
    let machines = parse_input(input, d);

    Some(machines.iter().fold(0, |acc, m| {
        let solution = solve(m);
        if solution.is_some() {
            let (a, b) = solution.unwrap();
            acc + (a * 3 + b)
        } else {
            acc
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // Ugh, no new test for part two
        assert_eq!(result, None);
    }
}
