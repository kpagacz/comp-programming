fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|coord| coord.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct OrderedFloat {
    inner: f64,
}

impl OrderedFloat {
    fn new(inner: f64) -> Self {
        Self { inner }
    }
}

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for OrderedFloat {}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.partial_cmp(&other.inner).unwrap()
    }
}

#[derive(Debug)]
struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parents: (0..size).collect(),
        }
    }

    fn root(&mut self, node: usize) -> usize {
        let mut root = node;
        while self.parents[root] != root {
            root = self.parents[root];
        }
        self.parents[node] = root;
        root
    }

    fn merge(&mut self, first: usize, second: usize) -> bool {
        let first_root = self.root(first);
        let second_root = self.root(second);

        if first_root != second_root {
            self.parents[second_root] = first_root;
            true
        } else {
            false
        }
    }

    fn tree_sizes(&mut self) -> Vec<usize> {
        let mut sizes = vec![0; self.parents.len()];
        for node in 0..self.parents.len() {
            sizes[self.root(node)] += 1;
        }
        sizes
    }
}

fn part1(input: &str, joins: usize) -> usize {
    let boxes = parse_input(input);
    use std::collections::BinaryHeap;
    let mut pq = BinaryHeap::new();

    for first in 0..boxes.len() {
        for second in first + 1..boxes.len() {
            let first_box = &boxes[first];
            let second_box = &boxes[second];
            let distance = straight_line_distance(first_box, second_box);
            pq.push(std::cmp::Reverse((distance, first, second)));
        }
    }

    let mut uf = UnionFind::new(boxes.len());
    for _ in 0..joins {
        if let Some(std::cmp::Reverse((_, first, second))) = pq.pop() {
            uf.merge(first, second);
        }
    }

    let mut sizes = uf.tree_sizes();
    sizes.sort_unstable();
    sizes.reverse();
    sizes[..3].iter().fold(1, |acc, size| acc * *size)
}

fn part2(input: &str) -> i64 {
    let boxes = parse_input(input);
    use std::collections::BinaryHeap;
    let mut pq = BinaryHeap::new();

    for first in 0..boxes.len() {
        for second in first + 1..boxes.len() {
            let first_box = &boxes[first];
            let second_box = &boxes[second];
            let distance = straight_line_distance(first_box, second_box);
            pq.push(std::cmp::Reverse((distance, first, second)));
        }
    }

    let mut uf = UnionFind::new(boxes.len());
    let mut last_merged = (0, 0);
    while let Some(std::cmp::Reverse((_, first, second))) = pq.pop() {
        if uf.merge(first, second) {
            last_merged = (first, second);
        }
    }

    boxes[last_merged.0][0] * boxes[last_merged.1][0]
}

fn straight_line_distance(first: &[i64], second: &[i64]) -> OrderedFloat {
    let x_sq = (first[0] - second[0]).pow(2) as f64;
    let y_sq = (first[1] - second[1]).pow(2) as f64;
    let z_sq = (first[2] - second[2]).pow(2) as f64;

    OrderedFloat::new((x_sq + y_sq + z_sq).sqrt())
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");
    println!("Part 1 (test): {} expected 40", part1(test, 10));
    println!("Part 1: {}", part1(input, 1000));
    println!("Part 2 (test): {} expected: 25272", part2(test));
    println!("Part 2: {}", part2(input));
}
