// https://leetcode.com/problems/path-with-maximum-probability/
pub struct Solution;

#[derive(PartialEq)]
struct PoorMansFloat(f64);
impl Eq for PoorMansFloat {}
impl PartialOrd for PoorMansFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PoorMansFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        use std::collections::HashMap;
        let edges = edges.into_iter().enumerate().fold(
            HashMap::<usize, Vec<(f64, usize)>>::new(),
            |mut map, (pos, edge)| {
                let (from, to) = (edge[0], edge[1]);
                map.entry(from as usize)
                    .or_default()
                    .push((succ_prob[pos], to as usize));
                map.entry(to as usize)
                    .or_default()
                    .push((succ_prob[pos], from as usize));
                map
            },
        );

        use std::collections::BinaryHeap;
        let start_node = start_node as usize;
        let end_node = end_node as usize;
        let n = n as usize;
        let mut visited = vec![false; n];
        let mut pq = BinaryHeap::new();
        pq.push((PoorMansFloat(1f64), start_node));

        while let Some((prob, node)) = pq.pop() {
            if visited[node] {
                continue;
            }
            if node == end_node {
                return prob.0;
            }
            visited[node] = true;

            for (edge_prob, dest) in edges.get(&node).unwrap_or(&Vec::new()) {
                let new_prob = PoorMansFloat(prob.0 * edge_prob);
                pq.push((new_prob, *dest));
            }
        }

        0 as _
    }
}

fn main() {
    println!("Hello, world!");
}
