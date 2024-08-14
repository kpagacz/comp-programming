// https://leetcode.com/problems/shortest-distance-after-road-addition-queries-i/
pub struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = (1..=n)
            .map(|destination| vec![destination])
            .collect::<Vec<_>>();

        let djikstra = |graph: &[Vec<usize>]| {
            use std::collections::BinaryHeap;

            let mut pq = BinaryHeap::from_iter([(0, 0)]);
            let mut visited = vec![false; n];

            while let Some((rev_distance, node)) = pq.pop() {
                let distance = -rev_distance;
                if node == n - 1 {
                    return distance;
                }
                if visited[node] {
                    continue;
                }
                visited[node] = true;

                for neighbour in graph[node].iter() {
                    pq.push((-(distance + 1), *neighbour));
                }
            }
            unreachable!()
        };

        let mut answer = Vec::with_capacity(queries.len());
        for query in queries {
            let (from, to) = (query[0] as usize, query[1] as usize);
            graph[from].push(to);

            answer.push(djikstra(&graph));
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
