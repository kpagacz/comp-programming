// https://leetcode.com/problems/count-the-number-of-complete-components/description/?envType=daily-question&envId=2025-03-22
pub struct Solution;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut parents = (0..n as usize).collect::<Vec<_>>();

        use std::collections::HashMap;

        fn find(parents: &mut [usize], node: usize) -> usize {
            if parents[node] == node {
                node
            } else {
                let root = find(parents, parents[node]);
                parents[node] = root;
                parents[node]
            }
        }

        fn merge(parents: &mut [usize], first: usize, second: usize) {
            let first_root = find(parents, first);
            let second_root = find(parents, second);

            if first_root != second_root {
                parents[second_root] = first_root;
            }
        }

        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

        for edge in edges {
            let (first, second) = (edge[0] as usize, edge[1] as usize);
            graph.entry(first).or_default().push(second);
            graph.entry(second).or_default().push(first);

            merge(&mut parents, first, second);
            // println!("first: {first} second: {second}");
            // println!("Merged: {parents:?}");
        }

        let mut components = vec![vec![]; n as usize];

        for node in 0..parents.len() {
            let root = find(&mut parents, node);
            components[root].push(node);
        }

        fn is_complete_component(component: &[usize], graph: &HashMap<usize, Vec<usize>>) -> bool {
            for i in 0..component.len() {
                if let Some(ns) = graph.get(&component[i]) {
                    for j in i + 1..component.len() {
                        if !ns.contains(&component[j]) {
                            return false;
                        }
                    }
                }
            }
            true
        }

        // println!("{graph:?}");
        // println!("comps: {components:?}");
        // println!("parents: {parents:?}");
        components
            .into_iter()
            .filter(|component| !component.is_empty() && is_complete_component(component, &graph))
            .count() as _
    }
}

fn main() {
    let test_cases = [(3, vec![vec![1, 0], vec![2, 0]])];
    for (n, edges) in test_cases {
        println!("Input: {n} {edges:?}");
        println!("{}", Solution::count_complete_components(n, edges));
    }
}
