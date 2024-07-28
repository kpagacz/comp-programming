// https://leetcode.com/problems/second-minimum-time-to-reach-destination/description/
pub struct Solution;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        use std::collections::BinaryHeap;
        let n = n as usize;
        let edges = edges
            .into_iter()
            .fold(vec![vec![]; n + 1], |mut acc, edge| {
                acc[edge[0] as usize].push(edge[1] as usize);
                acc[edge[1] as usize].push(edge[0] as usize);
                acc
            });
        let mut pq = BinaryHeap::from([(0i32, 1usize)]);
        let mut min_distance = vec![i32::MAX; n + 1];
        let mut seen = vec![0; n + 1];
        seen[1] = 1;

        // println!("{edges:?}");
        while let Some((time_elapsed, node)) = pq.pop() {
            let mut time_elapsed = -time_elapsed;

            if seen[node] == 2 {
                // If the node already had its second min, skip
                continue;
            }
            seen[node] = 1;
            if time_elapsed > min_distance[node] {
                // Mark the second min through the node
                seen[node] += 1;
            }
            if node == n && seen[node] == 2 {
                // found the second min to n
                return time_elapsed;
            }
            min_distance[node] = min_distance[node].min(time_elapsed);

            if (time_elapsed / change) % 2 == 1 {
                time_elapsed = change * (time_elapsed / change + 1);
            }

            edges[node].iter().for_each(|destination| {
                let new_time_elapsed = time_elapsed + time;
                pq.push((-new_time_elapsed, *destination));
            });
        }
        unreachable!()
    }
}

fn main() {
    let test_cases = [
        (
            5,
            vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]],
            3,
            5,
        ),
        (2, vec![vec![1, 2]], 3, 2),
    ];
    for (n, edges, time, change) in test_cases {
        println!("{}", Solution::second_minimum(n, edges, time, change));
    }
}
