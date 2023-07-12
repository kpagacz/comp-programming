// https://leetcode.com/problems/find-eventual-safe-states/
pub struct Solution {}

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut safe_nodes = vec![false; graph.len()];
        let mut visited = vec![false; graph.len()];

        for node in 0..graph.len() {
            Self::mark_safe_nodes(&graph, node as i32, &mut safe_nodes, &mut visited);
        }

        let mut safe_nodes_idxs = safe_nodes
            .iter()
            .enumerate()
            .filter(|(i, &safe)| safe)
            .map(|(id, _)| id as i32)
            .collect::<Vec<i32>>();
        safe_nodes_idxs.sort();
        safe_nodes_idxs
    }

    fn mark_safe_nodes(
        graph: &Vec<Vec<i32>>,
        current: i32,
        safe_nodes: &mut Vec<bool>,
        visited: &mut Vec<bool>,
    ) -> bool {
        if visited[current as usize] {
            return safe_nodes[current as usize];
        }
        visited[current as usize] = true;

        safe_nodes[current as usize] = graph[current as usize].iter().fold(true, |acc, &dest| {
            acc && Self::mark_safe_nodes(graph, dest, safe_nodes, visited)
        });

        safe_nodes[current as usize]
    }
}

fn main() {
    println!("Hello, world!");
}
