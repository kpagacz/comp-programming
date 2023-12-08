use std::collections::BTreeMap;
type Map = BTreeMap<String, (String, String)>;
fn parse_input(input: &str) -> (Vec<u8>, Map) {
    let lines: Vec<_> = input.lines().collect();
    let instructions = lines[0].as_bytes().to_owned();
    let nodes: Map = lines[2..]
        .iter()
        .map(|line| {
            let (node, children) = line.split_once(" = ").unwrap();
            let (child1, child2) = children
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap();
            (
                node.to_owned(),
                (String::from(child1), String::from(child2)),
            )
        })
        .collect();
    (instructions, nodes)
}

fn traverse_desert(instructions: &[u8], nodes: Map) -> usize {
    let mut current_node = "AAA";
    let end_node = "ZZZ";
    let mut it = 0;
    let mut times = 0;
    while current_node != end_node {
        it %= instructions.len();
        let entry = nodes.get(current_node).unwrap();
        current_node = if instructions[it] == b'L' {
            &entry.0
        } else {
            &entry.1
        };
        it += 1;
        times += 1;
    }
    times
}

fn part1(input: &str) -> usize {
    let (instructions, nodes) = parse_input(input);
    traverse_desert(&instructions, nodes)
}

fn find_end<'a>(mut node: &'a str, nodes: &'a Map, instructions: &[u8]) -> usize {
    let mut it = 0;
    let mut times = 0;
    while !node.ends_with('Z') {
        it %= instructions.len();
        let entry = nodes.get(node).unwrap();
        node = if instructions[it] == b'L' {
            &entry.0
        } else {
            &entry.1
        };
        it += 1;
        times += 1;
    }
    times
}

fn lcm(a: usize, b: usize) -> usize {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }
        gcd(b, a % b)
    }

    a * b / gcd(a, b)
}

fn part2(input: &str) -> usize {
    let (instructions, nodes) = parse_input(input);
    let starting_nodes: Vec<_> = nodes.keys().filter(|node| node.ends_with('A')).collect();
    // println!("Got {} starting nodes.", starting_nodes.len());
    starting_nodes
        .iter()
        .map(|node| find_end(node, &nodes, &instructions))
        .reduce(lcm)
        .unwrap()
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test));
    let test = include_str!("../testpart2");
    println!("Part 2: {}", part2(test));

    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
