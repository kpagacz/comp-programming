use rand::distributions::{Distribution, Uniform};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct Edge(usize, usize, i32);
fn parse_input(input: &str) -> (Vec<Edge>, BTreeMap<String, usize>) {
    let mut nodes = BTreeMap::new();
    (
        input
            .lines()
            .flat_map(|line| {
                let (source, destinations) = line.split_once(": ").unwrap();
                let destinations = destinations.split_whitespace().collect::<Vec<_>>();
                if !nodes.contains_key(source) {
                    nodes.insert(source.to_string(), nodes.len());
                }
                destinations.iter().for_each(|&destination| {
                    if !nodes.contains_key(destination) {
                        nodes.insert(destination.to_string(), nodes.len());
                    }
                });
                destinations
                    .into_iter()
                    .map(|destination| {
                        Edge(
                            *nodes.get(source).unwrap(),
                            *nodes.get(destination).unwrap(),
                            0,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
        nodes,
    )
}

#[derive(Debug, Clone)]
struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }

    fn find(&mut self, node: usize) -> usize {
        if self.parent[node] == node {
            node
        } else {
            self.parent[node] = self.find(self.parent[node]);
            self.parent[node]
        }
    }

    fn merge(&mut self, first: usize, second: usize) {
        let first_parent = self.find(first);
        let second_parent = self.find(second);
        self.parent[first_parent] = self.parent[second_parent];
    }

    fn count_set_sizes(&mut self) -> BTreeMap<usize, usize> {
        for i in 0..self.parent.len() {
            self.find(i);
        }
        self.parent.iter().fold(BTreeMap::new(), |mut map, parent| {
            *map.entry(*parent).or_insert(0) += 1;
            map
        })
    }
}

fn karger(mut edges: Vec<Edge>, n: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(1..10000);
    loop {
        edges
            .iter_mut()
            .for_each(|edge| edge.2 = uniform.sample(&mut rng));
        edges.sort_unstable_by_key(|edge| edge.2);
        let mut set_count = n;
        let mut dsu = Dsu::new(set_count);
        for edge in &edges {
            if dsu.find(edge.0) != dsu.find(edge.1) {
                set_count -= 1;
                dsu.merge(edge.0, edge.1);
            }
            if set_count == 2 {
                break;
            }
        }
        let mut different_sets = 0;
        for edge in &edges {
            if dsu.find(edge.0) != dsu.find(edge.1) {
                different_sets += 1;
            }
        }
        if different_sets <= 3 {
            return dsu.count_set_sizes().values().copied().collect();
        }
    }
}

fn print_graphviz_format(edges: &[Edge], n: usize) {
    println!("Number of nodes: {}", n);
    println!("Number of edges: {}", edges.len());
    println!("digraph G {{");
    for i in 0..n {
        println!("{i};");
    }
    for edge in edges {
        println!("{} -> {};", edge.0, edge.1);
    }
    println!("}}");
}

fn count_sets(edges: &[Edge], n: usize) -> (usize, usize) {
    let mut dsu = Dsu::new(n);
    for edge in edges {
        if dsu.find(edge.0) != dsu.find(edge.1) {
            dsu.merge(edge.0, edge.1);
        }
    }
    let set_sizes: Vec<_> = dsu.count_set_sizes().values().copied().collect();
    println!("{set_sizes:?}");
    (0, 0)
}

fn part1(input: &str) -> usize {
    let (edges, nodes) = parse_input(input);
    let sizes = karger(edges, nodes.len());
    sizes[0] * sizes[1]
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test));

    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
}
