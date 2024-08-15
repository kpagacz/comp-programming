// https://leetcode.com/problems/time-taken-to-mark-all-nodes/description/
pub struct Solution;

impl Solution {
    pub fn time_taken(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let graph = edges.into_iter().fold(vec![vec![]; n], |mut graph, edge| {
            let (from, to) = (edge[0] as usize, edge[1] as usize);
            graph[from].push(to);
            graph[to].push(from);
            graph
        });

        let mut max_distances = vec![(0, 0); n];
        let mut answer = vec![0; n];

        fn dfs1(
            node: usize,
            parent: usize,
            max_distances: &mut [(i32, i32)],
            graph: &[Vec<usize>],
        ) {
            for &child in &graph[node] {
                if child == parent {
                    continue;
                }
                dfs1(child, node, max_distances, graph);

                let child_distance = max_distances[child].0 + 2 - (child % 2) as i32;
                if child_distance > max_distances[node].0 {
                    max_distances[node].1 = max_distances[node].0;
                    max_distances[node].0 = child_distance;
                } else if child_distance > max_distances[node].1 {
                    max_distances[node].1 = child_distance;
                }
            }
        }
        dfs1(0, usize::MAX, &mut max_distances, &graph);

        fn dfs2(
            node: usize,
            parent: usize,
            to_parent: i32,
            max_distances: &[(i32, i32)],
            graph: &[Vec<usize>],
            answer: &mut [i32],
        ) {
            answer[node] = max_distances[node].0.max(to_parent);
            for &child in &graph[node] {
                if parent == child {
                    continue;
                }

                let child_distance = max_distances[child].0 + 2 - (child % 2) as i32;
                if child_distance == max_distances[node].0 {
                    dfs2(
                        child,
                        node,
                        i32::max(max_distances[node].1, to_parent) + 2 - (node % 2) as i32,
                        max_distances,
                        graph,
                        answer,
                    );
                } else {
                    dfs2(
                        child,
                        node,
                        answer[node] + 2 - (node % 2) as i32,
                        max_distances,
                        graph,
                        answer,
                    );
                }
            }
        }
        dfs2(0, usize::MAX, 0, &max_distances, &graph, &mut answer);

        answer
    }
}

fn main() {
    let test_cases = [
        vec![vec![0, 1], vec![0, 2]],
        vec![vec![0, 1]],
        vec![vec![2, 4], vec![0, 1], vec![2, 3], vec![0, 2]],
    ];

    for edges in test_cases {
        println!("{:?}", Solution::time_taken(edges));
    }
}
