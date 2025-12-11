use std::collections::HashMap;
fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line| {
            let (node, targets) = line.split_once(": ").unwrap();
            (
                node.to_owned(),
                targets.split_whitespace().map(str::to_owned).collect(),
            )
        })
        .fold(HashMap::new(), |mut map, (node, targets)| {
            map.insert(node, targets);
            map
        })
}

fn dfs(
    node: &str,
    target: &str,
    exclude: &str,
    graph: &HashMap<String, Vec<String>>,
    mem: &mut HashMap<String, usize>,
) -> usize {
    if node == target {
        return 1;
    }
    if let Some(answer) = mem.get(node) {
        return *answer;
    }

    let empty = Vec::new();
    let outputs: Vec<_> = graph
        .get(node)
        .unwrap_or(&empty)
        .iter()
        .filter(|node| *node != exclude)
        .collect();

    let mut acc = 0;
    for output in outputs {
        acc += dfs(output, target, exclude, graph, mem);
    }
    mem.insert(node.to_owned(), acc);
    acc
}

fn part1(input: &str) -> usize {
    let graph = parse_input(input);
    let mut mem = HashMap::new();
    dfs("you", "out", "madeup", &graph, &mut mem)
}

fn part2(input: &str) -> usize {
    let graph = parse_input(input);
    let mut mem = HashMap::new();
    let svr_to_dac = dfs("svr", "dac", "fft", &graph, &mut mem);
    let mut mem = HashMap::new();
    let svr_to_fft = dfs("svr", "fft", "dac", &graph, &mut mem);
    let mut mem = HashMap::new();
    let dac_to_fft = dfs("dac", "fft", "out", &graph, &mut mem);
    let mut mem = HashMap::new();
    let fft_to_dac = dfs("fft", "dac", "out", &graph, &mut mem);
    let mut mem = HashMap::new();
    let dac_to_out = dfs("dac", "out", "fft", &graph, &mut mem);
    let mut mem = HashMap::new();
    let fft_to_out = dfs("fft", "out", "dac", &graph, &mut mem);

    dbg!(svr_to_dac) * dbg!(dac_to_fft) * dbg!(fft_to_out)
        + dbg!(svr_to_fft) * dbg!(fft_to_dac) * dbg!(dac_to_out)
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");
    println!("Part 1 (test): {} expected: 5", part1(test));
    println!("Part 1: {}", part1(input));
    let test2 = include_str!("../test2");
    println!("Part 2 (test): {} expected: 2", part2(test2));
    println!("Part 2: {}", part2(input));
}
