fn parse_input(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once('-').unwrap();
            (first.to_owned(), second.to_owned())
        })
        .collect()
}

use std::collections::HashMap;
fn create_graph(connections: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for (first, second) in connections {
        graph.entry(first.clone()).or_default().push(second.clone());
        graph.entry(second).or_default().push(first);
    }
    graph
}

fn list_threes(graph: &HashMap<String, Vec<String>>) -> Vec<(String, String, String)> {
    let mut answer = vec![];
    for (node, dests) in graph.iter() {
        for i in 0..dests.len() {
            let first_dest = &dests[i];
            for second_dest in &dests[i + 1..] {
                if graph.get(first_dest).unwrap().contains(second_dest) {
                    answer.push((node.clone(), first_dest.clone(), second_dest.clone()));
                }
            }
        }
    }
    answer
}

fn part1(input: &str) -> usize {
    let connections = parse_input(input);
    let graph = create_graph(connections);

    let threes = list_threes(&graph);
    let mut answer = 0;
    for (first, second, third) in threes {
        if first.starts_with('t') || second.starts_with('t') || third.starts_with('t') {
            answer += 1;
        }
    }
    answer / 3
}

fn common_dests<'a>(
    initial: &[&'a String],
    second: &str,
    graph: &'a HashMap<String, Vec<String>>,
) -> Vec<&'a String> {
    let first_dests = initial;
    let second_dests = graph.get(second).unwrap();
    let mut common = vec![];
    for &first in first_dests {
        for second in second_dests {
            if first == second {
                common.push(first);
            }
        }
    }
    common
}

fn find_largest_lan_party(common: &[&String], graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    if common.is_empty() {
        return Vec::default();
    }

    let mut largest_lan_party = vec![];
    for i in 0..common.len() {
        let dest = common[i];
        let new_common_dests = common_dests(&common[i + 1..], dest, graph);
        let mut lan_party = find_largest_lan_party(&new_common_dests, graph);
        lan_party.push(dest.to_owned());
        if lan_party.len() > largest_lan_party.len() {
            largest_lan_party = lan_party;
        }
    }

    largest_lan_party
}

fn part2(input: &str) -> String {
    let connections = parse_input(input);
    let graph = create_graph(connections);

    let all_nodes: Vec<&String> = graph.keys().collect();
    let mut largest_lan = find_largest_lan_party(&all_nodes, &graph);
    largest_lan.sort_unstable();
    largest_lan.join(",")
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");

    println!("Part 1 (test): {}, expected 7", part1(test));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (test): {}, expected co,de,ka,ta", part2(test));
    println!("Part 2: {}", part2(input));
}
