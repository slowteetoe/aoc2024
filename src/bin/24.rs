use std::{
    collections::{BTreeMap, VecDeque},
    fs,
};

use derive_more::derive::Display;
use itertools::Itertools;
use petgraph::{dot::Dot, Graph};
use std::fmt::Display;
use tracing::debug;

advent_of_code::solution!(24);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
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

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?} {}", self.a, self.logic, self.b)
    }
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

/// turn into a graph and build a graphviz file
pub fn visualize(wires: BTreeMap<&str, bool>, gates: Vec<Gate>) -> Graph<String, String> {
    let mut graph = Graph::<String, String>::new();
    let mut nodemap = BTreeMap::new();

    for gate in gates.iter() {
        let op_node = graph.add_node(gate.logic.to_string()); // always add op node
        let a = *nodemap
            .entry(&gate.a)
            .or_insert_with(|| graph.add_node(gate.a.clone()));
        let b = *nodemap
            .entry(&gate.b)
            .or_insert_with(|| graph.add_node(gate.b.clone()));
        let out = *nodemap
            .entry(&gate.out)
            .or_insert_with(|| graph.add_node(gate.out.clone()));

        graph.extend_with_edges(&[(a, op_node), (b, op_node), (op_node, out)]);
    }
    let data = format!("{:?}", Dot::new(&graph));
    fs::write("data/images/day-24.dot", data).expect("Unable to write file");
    graph
}

pub fn part_one(input: &str) -> Option<u64> {
    let (wires, gates) = parse_input(input);
    let result = simulate(&wires, &gates);
    // println!("{:?}", result);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (wires, gates) = parse_input(input);
    visualize(wires, gates);
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

    #[traced_test]
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
