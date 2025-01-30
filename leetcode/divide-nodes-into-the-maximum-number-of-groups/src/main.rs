// https://leetcode.com/problems/divide-nodes-into-the-maximum-number-of-groups/description/
pub struct Solution;

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let graph: HashMap<i32, Vec<i32>> =
            edges.into_iter().fold(HashMap::new(), |mut graph, edge| {
                let (first, second) = (edge[0], edge[1]);
                graph.entry(first).or_default().push(second);
                graph.entry(second).or_default().push(first);
                graph
            });

        /// Returns:
        /// (whether it is bipartite, the id of the component, the candidate diameter)
        fn bfs(graph: &HashMap<i32, Vec<i32>>, start: i32, nodes: usize) -> (bool, usize, i32) {
            let mut visited = vec![false; nodes + 1];
            let mut id = usize::MAX;
            let mut queue = vec![start];
            let mut depth = 0;
            while !queue.is_empty() {
                let mut new_queue = vec![];

                while let Some(node) = queue.pop() {
                    if visited[node as usize] {
                        return (false, 0, 0);
                    }
                    id = id.min(node as usize);
                    visited[node as usize] = true;
                    new_queue.extend(
                        graph
                            .get(&node)
                            .unwrap_or(&Vec::default())
                            .iter()
                            .filter(|&&node| !visited[node as usize]),
                    );
                }

                new_queue.sort_unstable();
                new_queue.dedup();
                depth += 1;
                queue = new_queue;
            }
            (true, id, depth)
        }

        let mut diameters: HashMap<usize, i32> = HashMap::new();
        for node in 1..=n {
            let (feasible, id, depth) = bfs(&graph, node, n as usize);
            if !feasible {
                return -1;
            }
            diameters
                .entry(id)
                .and_modify(|m| *m = (*m).max(depth))
                .or_insert(depth);
        }
        diameters.values().sum::<i32>()
    }
}

fn main() {
    let test_cases = [
        (
            6,
            vec![
                vec![1, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 6],
                vec![2, 3],
                vec![4, 6],
            ],
        ),
        (
            26,
            vec![
                vec![9, 16],
                vec![8, 3],
                vec![20, 21],
                vec![12, 16],
                vec![14, 3],
                vec![7, 21],
                vec![22, 3],
                vec![22, 18],
                vec![11, 16],
                vec![25, 4],
                vec![2, 4],
                vec![14, 21],
                vec![23, 3],
                vec![17, 3],
                vec![2, 16],
                vec![24, 16],
                vec![13, 4],
                vec![10, 21],
                vec![7, 4],
                vec![9, 18],
                vec![14, 18],
                vec![14, 4],
                vec![14, 16],
                vec![1, 3],
                vec![25, 18],
                vec![17, 4],
                vec![1, 16],
                vec![23, 4],
                vec![2, 21],
                vec![5, 16],
                vec![24, 18],
                vec![20, 18],
                vec![19, 16],
                vec![24, 21],
                vec![9, 3],
                vec![24, 3],
                vec![19, 18],
                vec![25, 16],
                vec![19, 21],
                vec![6, 3],
                vec![26, 18],
                vec![5, 21],
                vec![20, 16],
                vec![2, 3],
                vec![10, 18],
                vec![26, 16],
                vec![8, 4],
                vec![11, 21],
                vec![23, 16],
                vec![13, 16],
                vec![25, 3],
                vec![7, 18],
                vec![19, 3],
                vec![20, 4],
                vec![26, 3],
                vec![23, 18],
                vec![15, 18],
                vec![17, 18],
                vec![10, 16],
                vec![26, 21],
                vec![23, 21],
                vec![7, 16],
                vec![8, 18],
                vec![10, 4],
                vec![24, 4],
                vec![7, 3],
                vec![11, 18],
                vec![9, 4],
                vec![26, 4],
                vec![13, 21],
                vec![22, 16],
                vec![22, 21],
                vec![20, 3],
                vec![6, 18],
                vec![9, 21],
                vec![10, 3],
                vec![22, 4],
                vec![1, 18],
                vec![25, 21],
                vec![11, 4],
                vec![1, 21],
                vec![15, 3],
                vec![1, 4],
                vec![15, 16],
                vec![2, 18],
                vec![13, 3],
                vec![8, 21],
                vec![13, 18],
                vec![11, 3],
                vec![15, 21],
                vec![8, 16],
                vec![17, 16],
                vec![15, 4],
                vec![12, 3],
                vec![6, 4],
                vec![17, 21],
                vec![5, 18],
                vec![6, 16],
                vec![6, 21],
                vec![12, 4],
                vec![19, 4],
                vec![5, 3],
                vec![12, 21],
                vec![5, 4],
            ],
        ),
    ];

    for (n, edges) in test_cases {
        println!("{}", Solution::magnificent_sets(n, edges));
    }
}
