use std::collections::HashMap;
fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut tree: HashMap<String, Vec<String>> = HashMap::default();
    input.lines().for_each(|line| {
        let (from, to) = line.split_once(":").unwrap();
        if from == "BUG" || from == "ANT" {
            return;
        }
        to.split(",")
            .filter(|&dest| dest != "ANT" && dest != "BUG")
            .for_each(|dest| {
                tree.entry(from.to_owned())
                    .or_default()
                    .push(dest.to_owned());
            });
    });
    tree
}

fn workhorse<R>(input: &str, nodes_reduce: R) -> String
where
    R: Fn(&[String]) -> String,
{
    fn rec<R>(
        tree: &HashMap<String, Vec<String>>,
        current_path: &mut Vec<String>,
        length_to_path: &mut HashMap<usize, Vec<String>>,
        nodes_reduce: &R,
    ) where
        R: Fn(&[String]) -> String,
    {
        if current_path.is_empty() {
            return;
        }

        if current_path.last().unwrap() == "@" {
            length_to_path
                .entry(current_path.len())
                .or_default()
                .push(nodes_reduce(current_path));
            return;
        }

        if let Some(children) = tree.get(current_path.last().unwrap()) {
            for dest in children {
                current_path.push(dest.to_owned());
                rec(tree, current_path, length_to_path, nodes_reduce);
                current_path.pop();
            }
        }
    }

    let tree = parse_input(input);
    let mut current_path = vec!["RR".to_owned()];
    let mut length_to_path = HashMap::default();
    rec(&tree, &mut current_path, &mut length_to_path, &nodes_reduce);

    length_to_path
        .values()
        .find(|paths| paths.len() == 1)
        .unwrap()[0]
        .clone()
}

fn part1(input: &str) -> String {
    fn concat_reduce(nodes: &[String]) -> String {
        nodes.concat()
    }
    workhorse(input, concat_reduce)
}

fn part2(input: &str) -> String {
    fn first_letter_reduce(nodes: &[String]) -> String {
        nodes
            .iter()
            .map(|node| node.chars().next().unwrap())
            .collect::<String>()
    }

    workhorse(input, first_letter_reduce)
}

fn main() {
    let input0 = include_str!("./input0");
    let test0 = include_str!("./test0");
    println!("Part 1 (test): {} Expected: RRB@", part1(test0));
    println!("Part 1: {}", part1(input0));

    let input1 = include_str!("../input1");
    println!("Part 2: {}", part2(input1));

    let input2 = include_str!("../input2");
    println!("Part 3: {}", part2(input2));
}
