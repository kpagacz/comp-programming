// https://leetcode.com/problems/valid-arrangement-of-pairs/description/?envType=daily-question&envId=2024-11-30
pub struct Solution;

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::with_capacity(pairs.len() * 2);
        for pair in &pairs {
            // The graph is directed
            adj.entry(pair[0]).or_default().push(pair[1]);
        }

        let mut degrees = HashMap::with_capacity(pairs.len() * 2);
        for pair in &pairs {
            degrees.entry(pair[0]).or_insert((0, 0)).1 += 1;
            degrees.entry(pair[1]).or_insert((0, 0)).0 += 1;
        }

        // Find the start
        // If all degrees are even, then this is a Eulerian cycle
        // so it does not matter where it starts
        let start = degrees
            .iter()
            .filter(|(_, &deg)| (deg.0 + deg.1) % 2 == 1 && deg.1 > deg.0)
            .filter(|(key, _)| adj.contains_key(key))
            .map(|(&key, _)| key)
            .next()
            .unwrap_or(*adj.keys().next().unwrap());

        // Find the path
        let mut stack = vec![start];
        let mut path = vec![];

        while let Some(next) = stack.last() {
            if let Some(neighbors) = adj.get_mut(next) {
                if !neighbors.is_empty() {
                    let next_node = neighbors.pop().unwrap();
                    stack.push(next_node);
                } else {
                    path.push(stack.pop().unwrap());
                }
            } else {
                path.push(stack.pop().unwrap());
            }
        }

        path.windows(2)
            .rev()
            .map(|window| {
                let mut v = window.to_vec();
                v.reverse();
                v
            })
            .collect()
    }
}

fn main() {
    let test_cases = [
        vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]],
        vec![vec![1, 3], vec![3, 2], vec![2, 1]],
        vec![vec![1, 2], vec![1, 3], vec![2, 1]],
        vec![vec![17, 18], vec![18, 10], vec![10, 18]],
    ];
    for pairs in test_cases {
        println!("{:?}", Solution::valid_arrangement(pairs));
    }
}
