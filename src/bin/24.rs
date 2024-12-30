use std::collections::{BTreeMap, VecDeque};

use itertools::Itertools;
use tracing::debug;

advent_of_code::solution!(24);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Logic {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Gate<'a> {
    logic: Logic,
    a: &'a str,
    b: &'a str,
    out: &'a str,
}

fn parse_input(input: &str) -> (BTreeMap<&str, bool>, Vec<Gate>) {
    let (wires, gates) = input.split_once("\n\n").expect("two sections");
    let wires: BTreeMap<_, _> = wires
        .lines()
        .map(|line| {
            let (name, val) = line.split_once(": ").unwrap();
            (name, val == "1")
        })
        .collect();

    let gates = gates
        .lines()
        .map(|line| {
            // gate logic gate -> out
            let (left, out) = line.split_once(" -> ").expect("two gate parts");
            let left = left.splitn(3, " ").collect_vec();
            Gate {
                logic: match left[1] {
                    "AND" => Logic::And,
                    "OR" => Logic::Or,
                    "XOR" => Logic::Xor,
                    _ => unreachable!(),
                },
                a: left[0],
                b: left[2],
                out,
            }
        })
        .collect_vec();

    debug!(?wires, ?gates);
    (wires, gates)
}

fn simulate(wires: &BTreeMap<&str, bool>, gates: &Vec<Gate>) -> u64 {
    let mut wires = wires.clone();
    let mut queue = VecDeque::from_iter(gates.iter());

    while let Some(gate) = queue.pop_front() {
        match (wires.get(gate.a), wires.get(gate.b)) {
            (Some(&a), Some(&b)) => {
                // compute and "write" to output wire
                let result = match gate.logic {
                    Logic::And => a && b,
                    Logic::Or => a || b,
                    Logic::Xor => a ^ b,
                };
                wires.insert(gate.out, result);
            }
            _ => {
                // println!("{:?} not ready yet", &gate);
                //push back onto queue if one or more wires values aren't available yet
                queue.push_back(gate);
            }
        }
    }
    let final_bin = wires
        .iter()
        .filter(|w| w.0.starts_with("z"))
        .sorted()
        .rev()
        .map(|v| if *v.1 { "1" } else { "0" })
        .join("");
    let output = u64::from_str_radix(&final_bin, 2).expect("valid binary");
    debug!(final_bin, output);
    output
}

pub fn part_one(input: &str) -> Option<u64> {
    let (wires, gates) = parse_input(input);
    let result = simulate(&wires, &gates);
    // println!("{:?}", result);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[traced_test]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
