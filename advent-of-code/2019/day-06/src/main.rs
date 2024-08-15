// https://adventofcode.com/2019/day/6
use std::collections::HashMap;
fn parse(input: &str) -> HashMap<String, Vec<&str>> {
    input.lines().fold(HashMap::new(), |mut map, line| {
        let (from, to) = line.split_once(')').unwrap();
        map.entry(from.to_string()).or_default().push(to);

        map
    })
}

fn parse2(input: &str) -> HashMap<String, Vec<&str>> {
    input.lines().fold(HashMap::new(), |mut map, line| {
        let (from, to) = line.split_once(')').unwrap();
        map.entry(from.to_owned()).or_default().push(to);
        map.entry(to.to_string()).or_default().push(from);

        map
    })
}

fn part1(input: &str) -> usize {
    let graph = parse(input);

    fn dfs(node: &str, ancestors: usize, graph: &HashMap<String, Vec<&str>>) -> usize {
        ancestors
            + graph
                .get(node)
                .unwrap_or(&Vec::default())
                .iter()
                .map(|&child| dfs(child, ancestors + 1, graph))
                .sum::<usize>()
    }

    dfs("COM", 0, &graph)
}

fn part2(input: &str) -> usize {
    let graph = parse2(input);

    fn dfs(
        node: &str,
        ancestors: usize,
        parent: &str,
        graph: &HashMap<String, Vec<&str>>,
    ) -> usize {
        if node == "SAN" {
            return ancestors - 1;
        }

        let empty_vec = Vec::default();
        let children = graph
            .get(node)
            .unwrap_or(&empty_vec)
            .iter()
            .filter(|&&node| node != parent);
        if children.clone().count() == 0 {
            return usize::MAX;
        }
        graph
            .get(node)
            .unwrap_or(&Vec::default())
            .iter()
            .filter(|&&node| node != parent)
            .map(|&child| dfs(child, ancestors + 1, node, graph))
            .min()
            .unwrap()
    }

    dfs("YOU", 0, "NOTHING", &graph) - 1
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");
    let test2 = include_str!("../test2");
    println!("Part 1 (test): {}", part1(test));
    println!("Part 2 (test): {}", part2(test2));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
