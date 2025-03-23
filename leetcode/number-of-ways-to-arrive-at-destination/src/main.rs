// https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/description
pub struct Solution;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;
        const MOD: i64 = 1_000_000_007;
        let mut pq = BinaryHeap::new();
        let mut cnt = vec![0; n as usize];
        let mut times = vec![i64::MAX; n as usize];
        let mut graph: HashMap<usize, Vec<(usize, i32)>> = HashMap::new();

        for road in roads {
            let (first, second, time) = (road[0] as usize, road[1] as usize, road[2]);

            graph.entry(first).or_default().push((second, time));
            graph.entry(second).or_default().push((first, time));
        }

        pq.push(Reverse((0, 0, 0))); // (time, node, from)
        while let Some(Reverse((time, node, from))) = pq.pop() {
            match time.cmp(&times[node]) {
                std::cmp::Ordering::Less => {
                    cnt[node] = cnt[from];
                    times[node] = time;

                    if let Some(ns) = graph.get(&node) {
                        for &(n, cost) in ns {
                            pq.push(Reverse((time + cost as i64, n, node)));
                        }
                    }
                }
                std::cmp::Ordering::Equal => cnt[node] = (cnt[node] + cnt[from]) % MOD,
                std::cmp::Ordering::Greater => continue,
            }
        }
        cnt[n as usize - 1] as _
    }
}

fn main() {
    let test_cases = [(
        7,
        vec![
            vec![0, 6, 7],
            vec![0, 1, 2],
            vec![1, 2, 3],
            vec![1, 3, 3],
            vec![6, 3, 3],
            vec![3, 5, 1],
            vec![6, 5, 1],
            vec![2, 5, 1],
            vec![0, 4, 5],
            vec![4, 6, 2],
        ],
    )];

    for (n, roads) in test_cases {
        println!("{}", Solution::count_paths(n, roads));
    }
}
