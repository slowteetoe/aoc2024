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
struct Gate {
    logic: Logic,
    a: String,
    b: String,
    out: String,
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
                a: left[0].to_owned(),
                b: left[2].to_owned(),
                out: out.to_owned(),
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
        match (wires.get(gate.a.as_str()), wires.get(gate.b.as_str())) {
            (Some(&a), Some(&b)) => {
                // compute and "write" to output wire
                let result = match gate.logic {
                    Logic::And => a && b,
                    Logic::Or => a || b,
                    Logic::Xor => a ^ b,
                };
                wires.insert(gate.out.as_str(), result);
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

fn normalize(gates: &mut Vec<Gate>) {
    let mut remaps = BTreeMap::<String, String>::new();
    let mut queue = VecDeque::from_iter(gates.iter_mut());

    let mut guard = 0;
    while let Some(g) = queue.pop_front() {
        if guard > 10000 {
            break;
        }
        guard += 1;
        match (
            g.a.starts_with("x"),
            g.b.starts_with("y"),
            g.out.starts_with("z"),
        ) {
            (false, false, false) => queue.push_back(g),
            (true, true, true) => continue,
            (true, true, false) => {
                if remaps.contains_key(&g.out) {
                    g.out = remaps.get(&g.out).unwrap().to_owned();
                    continue;
                } else {
                    remaps.insert(g.out.clone(), g.a.replace("x", "z"));
                    queue.push_back(g);
                }
            }
            (true, false, true) => {
                if remaps.contains_key(&g.b) {
                    g.b = remaps.get(&g.b).unwrap().to_owned();
                } else {
                    remaps.insert(g.b.clone(), g.a.replace("x", "y"));
                    queue.push_back(g);
                }
            }
            (false, true, true) => {
                if remaps.contains_key(&g.out) {
                    g.out = remaps.get(&g.out).unwrap().to_owned();
                } else {
                    remaps.insert(g.a.clone(), g.b.replace("y", "x"));
                    queue.push_back(g);
                }
            }
            (false, false, true) => {
                if remaps.contains_key(&g.a) {
                    g.a = remaps.get(&g.a).unwrap().to_owned();
                }
                if remaps.contains_key(&g.b) {
                    g.b = remaps.get(&g.b).unwrap().to_owned();
                }
                queue.push_back(g);
            }
            _ => {
                println!("don't know what to do with {:?}", g);
                queue.push_back(g);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (wires, gates) = parse_input(input);
    let result = simulate(&wires, &gates);
    // println!("{:?}", result);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (wires, mut gates) = parse_input(input);
    println!("BEFORE {:?}", &gates);
    normalize(&mut gates);
    println!("AFTER {:?}", &gates);
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
        let result = part_two(
            r"x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00",
        );
        assert_eq!(result, None);
    }
}
