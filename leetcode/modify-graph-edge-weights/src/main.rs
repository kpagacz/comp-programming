// https://leetcode.com/problems/modify-graph-edge-weights/description/
pub struct Solution;

impl Solution {
    pub fn modified_graph_edges(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        // graph map with edge ref
        let mut graph = edges.iter().fold(
            vec![vec![i32::MAX; n as usize]; n as usize],
            |mut adj, edge| {
                let (first, second, weight) = (edge[0] as usize, edge[1] as usize, edge[2]);
                adj[first][second] = if weight == -1 { 1 } else { weight };
                adj[second][first] = if weight == -1 { 1 } else { weight };
                adj
            },
        );
        let mutable_edges = edges.iter().fold(
            vec![vec![false; n as usize]; n as usize],
            |mut mem, edge| {
                if edge[2] == -1 {
                    mem[edge[0] as usize][edge[1] as usize] = true;
                    mem[edge[1] as usize][edge[0] as usize] = true;
                }
                mem
            },
        );

        let source = source as usize;
        let destination = destination as usize;

        let (mut shortest_length, mut predecessesors) = Self::djikstra(&graph, source, destination);
        match shortest_length.cmp(&target) {
            // The shortest path contains some modifiable nodes
            std::cmp::Ordering::Less => {
                while shortest_length < target {
                    let mut path = vec![destination];
                    while *path.last().unwrap() != source {
                        let last = *path.last().unwrap();
                        path.push(predecessesors[last]);
                    }

                    let mut mutable_path_edges: Vec<_> = path
                        .windows(2)
                        .filter(|edge| mutable_edges[edge[0]][edge[1]])
                        .collect();

                    if mutable_path_edges.is_empty() {
                        // The shortest path does not contain a mutable edge
                        return vec![];
                    }
                    let last_mutable_edge = mutable_path_edges.pop().unwrap();
                    let (first, second) = (last_mutable_edge[0], last_mutable_edge[1]);
                    graph[first][second] += target - shortest_length;
                    graph[second][first] += target - shortest_length;
                    (shortest_length, predecessesors) = Self::djikstra(&graph, source, destination);
                }
                edges
                    .iter()
                    .map(|edge| vec![edge[0], edge[1], graph[edge[0] as usize][edge[1] as usize]])
                    .collect()
            }
            // The shortest path is already equal to the target
            std::cmp::Ordering::Equal => edges
                .iter()
                .map(|edge| {
                    if edge[2] == -1 {
                        vec![edge[0], edge[1], 1]
                    } else {
                        edge.clone()
                    }
                })
                .collect(),
            // The shortest path cannot be smaller than it already is
            std::cmp::Ordering::Greater => vec![],
        }
    }

    fn djikstra(graph: &[Vec<i32>], source: usize, destination: usize) -> (i32, Vec<usize>) {
        // Djikstra with predecessesors and if weight == -1, treat it as -1
        let mut previous = vec![usize::MAX; graph.len()];
        let mut visited = vec![false; graph.len()];
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, source, source)));

        while let Some(Reverse((distance, node, previous_node))) = pq.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            previous[node] = previous_node;

            if node == destination {
                return (distance, previous);
            }

            graph[node]
                .iter()
                .enumerate()
                .filter(|(_, &edge_weight)| edge_weight != i32::MAX)
                .for_each(|(dest, &edge_weight)| {
                    pq.push(Reverse((distance.saturating_add(edge_weight), dest, node)));
                })
        }

        unreachable!()
    }
}

fn main() {
    let test_cases = [(
        5,
        vec![
            vec![4, 1, -1],
            vec![2, 0, -1],
            vec![0, 3, -1],
            vec![4, 3, -1],
        ],
        0,
        1,
        5,
        vec![vec![4, 1, 1], vec![2, 0, 1], vec![0, 3, 3], vec![4, 3, 1]],
    )];

    for (n, edges, source, destination, target, expected) in test_cases {
        println!(
            "Got {:?}\nExpected: {:?}",
            Solution::modified_graph_edges(n, edges, source, destination, target),
            expected
        );
    }
}
