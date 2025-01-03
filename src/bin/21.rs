use std::iter::once;

use itertools::Itertools;
use memoize::memoize;
use pathfinding::prelude::astar;
use tracing::instrument;

advent_of_code::solution!(21);

/// yeah, this is probably not the best either...  maybe building up reachable paths from each char would be easier
#[memoize]
fn next_step_on_numeric_keypad(start: char, end: char) -> Vec<char> {
    let (path, _cost) = astar(
        &(start, 'X'),
        |p| match p.0 {
            'A' => vec![(('0', '<'), 1), (('3', '^'), 1)],
            '0' => vec![(('2', '^'), 1), (('A', '>'), 1)],
            '1' => vec![(('4', '^'), 1), (('2', '>'), 1)],
            '2' => vec![
                (('1', '<'), 1),
                (('3', '>'), 1),
                (('5', '^'), 1),
                (('0', 'v'), 1),
            ],
            '3' => vec![(('2', '<'), 1), (('6', '^'), 1), (('A', 'v'), 1)],
            '4' => vec![(('5', '>'), 1), (('7', '^'), 1)],
            '5' => vec![
                (('4', '<'), 1),
                (('6', '>'), 1),
                (('8', '^'), 1),
                (('2', 'v'), 1),
            ],
            '6' => vec![(('5', '<'), 1), (('9', '^'), 1), (('3', 'v'), 1)],
            '7' => vec![(('8', '>'), 1), (('4', 'v'), 1)],
            '8' => vec![(('7', '<'), 1), (('9', '>'), 1), (('5', 'v'), 1)],
            '9' => vec![(('8', '<'), 1), (('6', 'v'), 1)],
            'X' => vec![],
            _ => unreachable!(),
        },
        |_| 0,
        |p| p.0 == end,
    )
    .expect("should have been a path");
    path.iter().map(|n| n.1).collect()
}

// let's stick with our inefficient searching for now...
#[memoize]
fn next_step_on_directional_keypad(start: char, end: char) -> Vec<char> {
    let (path, _cost) = astar(
        &(start, 'A'),
        |p| match p.0 {
            'A' => vec![(('^', '<'), 1), (('>', 'v'), 1)],
            '<' => vec![(('v', '>'), 1)],
            '^' => vec![(('v', 'v'), 1), (('A', '>'), 1)],
            '>' => vec![(('v', '<'), 1), (('A', '^'), 1)],
            'v' => vec![(('<', '<'), 1), (('^', '^'), 1), (('>', '>'), 1)],
            _ => unreachable!(),
        },
        |_| 2,
        |p| p.0 == end,
    )
    .expect("should have been a path");
    path.iter().map(|n| n.1).collect()
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c).collect())
        .collect()
}

fn code_to_numerical_movements(code: &Vec<char>) -> String {
    let mut movements = vec![];
    once(&'A')
        .chain(code)
        .tuple_windows()
        .for_each(|(start, end)| {
            if start == end {
                movements.push('A');
            } else {
                let mut next = next_step_on_numeric_keypad(*start, *end);
                // debug!("going from {start} to {end} is {:?}", next);
                next = next[1..].to_vec();
                next.push('A');
                movements.append(&mut next);
            }
        });
    movements.iter().join("")
}

fn numerical_to_directional_movements(numerical_movements: &str) -> String {
    let mut movements = vec![];
    once('A')
        .chain(numerical_movements.chars())
        .tuple_windows()
        .for_each(|(start, end)| {
            let mut next = next_step_on_directional_keypad(start, end);
            // wth?
            // println!("going from {end} to {start} is {:?}", next);
            if next.len() != 0 {
                next = next[1..].to_vec();
            }
            next.push('A');
            movements.append(&mut next);
        });
    movements.iter().join("")
}

// fn push_buttons(input: Vec<char>) -> String {
//     // let mut nodes = BTreeMap::<char, u32>::new();
//     // nodes.insert('A', 0);
//     // nodes.insert('^', 1);
//     // nodes.insert('>', 2);
//     // nodes.insert('v', 3);
//     // nodes.insert('<', 4);
//     let g = DiGraphMap::<&str, ()>::from_edges(&[
//         // (0, 1),
//         // (0, 2), // A
//         // (1, 3),
//         // (1, 0), // ^
//         // (2, 0),
//         // (2, 3), // >
//         // (3, 4),
//         // (3, 1),
//         // (3, 2), // v
//         // (4, 3), // <
//         ("A", "^"),
//         ("A", ">"),
//         ("^", "A"),
//         ("^", "v"),
//         (">", "A"),
//         (">", "v"),
//         ("v", "<"),
//         ("v", "^"),
//         ("v", ">"),
//         ("<", "v"),
//     ]);
//     println!("{:?}", Dot::new(&g));
//     let mut curr = g.nodes().find(|n| *n == "A").expect("root node");
//     let output: Vec<_> = input
//         .iter()
//         .map(|c| {
//             let c = c.to_string();
//             if c == "A" {
//                 curr
//             } else {
//                 let mut outbound = g.edges_directed(curr, petgraph::Direction::Outgoing);
//                 println!("on {}, looking at {:?} for {}", curr, outbound, c);
//                 if let Some(outbound_node) = outbound.find(|e| e.0 == curr && e.1 == c) {
//                     curr = outbound_node.1;
//                     outbound_node.0
//                 } else {
//                     panic!("nope")
//                 }
//             }
//         })
//         .collect();
//     output.iter().join("")
// }

#[instrument(skip(input))]
pub fn part_one(input: &str) -> Option<u32> {
    let codes = parse(input);
    let solutions = codes
        .iter()
        .map(|code| (code, code_to_numerical_movements(&code)))
        .inspect(|(code, m)| {
            println!("{:?}: {:?}", code, m);
        })
        .map(|(code, m)| (code, numerical_to_directional_movements(&m)))
        .inspect(|(code, m)| {
            println!("{:?}: {:?}", code, m);
        })
        .map(|(code, m)| (code, numerical_to_directional_movements(&m)))
        .inspect(|(code, m)| {
            println!("{:?}: {:?}", code, m);
        })
        .collect_vec();
    // debug!(?third);
    // let solutions = (0..solutions.iter().len())
    //     .map(|n| (codes[n].clone(), solutions[n].clone()))
    //     .collect::<BTreeMap<_, _>>();
    // debug!(?solutions);
    let complexity: u32 = solutions
        .iter()
        .inspect(|(code, seq)| {
            println!("{:?} => {:?}", code, seq);
        })
        .map(|(code, seq)| {
            (
                code.iter()
                    .filter(|c| **c != 'A')
                    .join("")
                    .parse::<u32>()
                    .expect("should have been valid number"),
                seq.chars().count() as u32,
            )
        })
        .inspect(|(num, seq_len)| {
            println!("{} => {}", num, seq_len);
        })
        .map(|(num, seq_len)| num * seq_len)
        .sum();
    Some(complexity)
}

#[instrument(skip(input))]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use tracing_test::traced_test;

    use super::*;

    // seems like we always generate the same shortest path,
    // even though there are several equivalent paths
    #[rstest]
    #[case("029A", "<A^A^^>AvvvA")]
    #[case("980A", "^^^A<AvvvA>A")]
    #[case("179A", "<^<A^^A>>AvvvA")]
    #[case("456A", "<^^<A>A>AvvA")]
    #[case("379A", "^A^^<<A>>AvvvA")]
    #[traced_test]
    #[test]
    fn test_code_to_numerical_movements(#[case] code: &str, #[case] expected: &str) {
        let code = code.to_owned().chars().collect_vec();
        let result = code_to_numerical_movements(&code);
        assert_eq!(result, expected);
        assert_eq!(expected.len(), result.len())
    }

    #[traced_test]
    #[test]
    fn test_numerical_to_movements() {
        let result = numerical_to_directional_movements("<A^A>^^AvvvA");
        // these are all valid paths
        assert!([
            "v<<A>^>A<A>A<AAv>A^Av<AAA^>A",
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A",
            "v<<A>^>A<A>AvA<^AA>Av<AAA^>A"
        ]
        .contains(&result.as_str()));
        assert_eq!(result, "v<<A>^>A<A>AvA<^AA>Av<AAA^>A")
    }

    #[rstest]
    #[test]
    #[case("v<<A>^>A<A>A<AAv>A^Av<AAA^>A", 68)]
    #[case("v<<A>>^A<A>AvA<^AA>A<vAAA>^A", 68)]
    #[case("v<<A>^>A<A>AvA<^AA>Av<AAA^>A", 68)]
    fn test_shortest_paths_have_same_lens(#[case] input: &str, #[case] expected_len: usize) {
        // ?! case 2 and 3 are equivalent paths, but when I generate this path, case 2 has 68 chars and case 3 has 70 chars, how is that even possible/
        let result = numerical_to_directional_movements(input);
        assert_eq!(
            result.len(),
            expected_len,
            "input of {} generated {}",
            input,
            result
        );
    }

    #[traced_test]
    #[rstest]
    #[test]
    #[case("v<<A>>^A<A>AvA<^AA>A<vAAA>^A", vec![
        "v<A<AA>^>AvAA<^A>Av<<A>^>AvA^Av<A^>Av<<A>^A>AAvA^Av<<A>A^>AAAvA<^A>A",
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
    ])]
    //"v<<A>^>A<A>A<AAv>A^Av<AAA^>A"
    // AAAAAAAAARGH!!!!
    #[case("v<<A>^>A", vec!["v<A<AA>^>AvA<^Av>A^A"])]
    #[case("v<<A>^>A<", vec!["v<A<AA>^>AvA<^Av>A^Av<<A"])]
    #[case("v<<A>^>A<AA", vec!["v<A<AA>^>AvA<^Av>A^Av<<A>^>AA"])]
    #[case("v<<A>^>A<AAv>A", vec!["v<A<AA>^>AvA<^Av>A^Av<<A>^>AAv<A>A^A"])]
    #[case("v<<A>^>A<AAv>A^A", vec!["v<A<AA>^>AvA<^Av>A^Av<<A>^>AAv<A>A^A<A>A"])]
    #[case("v<<A>^>A<AAv>A^Av<AAA", vec!["v<A<AA>^>AvA<^Av>A^Av<<A>^>AAv<A>A^A<A>Av<A<A>^>AAA"])]
    #[case("v<<A>^>A<AAv>A^Av<AAAvA", vec!["v<A<AA>^>AvA<^Av>A^Av<<A>^>AAv<A>A^A<A>Av<A<A>^>AAAv<A^>A"])]
    #[case("v<<A>^>A<AAv>A^Av<AAAvA<^A", vec!["v<A<AA>^>AvA<^Av>A^Av<<A>^>AAv<A>A^A<A>Av<A<A>^>AAAv<A^>Av<<A>^A>A"])]
    #[case("v<<A>^>A<AAv>A^Av<AAAvA<^A>A", vec!["v<A<AA>^>AvA<^Av>A^Av<<A>^>AAv<A>A^A<A>Av<A<A>^>AAAv<A^>Av<<A>^A>AvA^A"])]
    // here's what we actualy see for 029A
    #[case("v<<A>^>A<A>AvA<^AA>Av<AAA^>A", vec![""])]
    fn test_second_round_of_movements(#[case] input: &str, #[case] possible_solutions: Vec<&str>) {
        let result = numerical_to_directional_movements(input);
        // equivalent paths
        assert!(
            possible_solutions.contains(&result.as_str()),
            "{} was actually {}",
            input,
            result
        )
    }

    #[traced_test]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
