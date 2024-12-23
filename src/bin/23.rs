use std::collections::{BTreeMap, HashSet};

use itertools::Itertools;
use tracing::{debug, info, instrument};

advent_of_code::solution!(23);

fn find_triads(graph: &BTreeMap<String, HashSet<String>>) -> Vec<(String, String, String)> {
    let mut results = vec![];
    for (k, v) in graph.iter() {
        for connected_node in v.iter() {
            let v2 = graph.get(connected_node).unwrap();
            let in_common = v2.intersection(v);

            info!(current_node = k, looking_at = ?connected_node, ?v, ?v2, ?in_common);
            for ic in in_common {
                if !(k.starts_with("t") || connected_node.starts_with("t") || ic.starts_with("t")) {
                    continue;
                }
                let mut result = vec![k, connected_node, ic];
                result.sort();
                results.push((
                    result[0].to_owned(),
                    result[1].to_owned(),
                    result[2].to_owned(),
                ));
            }
        }
    }
    results.into_iter().unique().collect_vec()
}

fn parse_input(input: &str) -> BTreeMap<String, HashSet<String>> {
    input.lines().fold(BTreeMap::new(), |mut acc, line| {
        let (lh, rh) = line.trim().split_once("-").expect("split on -");

        acc.entry(lh.to_string())
            .and_modify(|v| {
                v.insert(rh.to_string());
            })
            .or_insert({
                let mut h = HashSet::new();
                h.insert(rh.to_string());
                h
            });

        acc.entry(rh.to_string())
            .and_modify(|v| {
                v.insert(lh.to_string());
            })
            .or_insert({
                let mut h = HashSet::new();
                h.insert(lh.to_string());
                h
            });

        acc
    })
}

#[instrument(skip(input))]
pub fn part_one(input: &str) -> Option<u32> {
    let network_map = parse_input(input);
    let result = find_triads(&network_map);
    debug!(?network_map, ?result);
    Some(result.len() as u32)
}

#[instrument(skip(_input))]
pub fn part_two(_input: &str) -> Option<u32> {
    // now we need the largest connected component, and to know which vertices those are
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
