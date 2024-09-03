// https://leetcode.com/problems/k-th-nearest-obstacle-queries/description/
pub struct Solution;

impl Solution {
    pub fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;
        let k = k as usize;
        let mut pq = BinaryHeap::new();
        queries
            .into_iter()
            .map(|query| {
                let distance = query[0].abs() + query[1].abs();
                pq.push(distance);
                while pq.len() > k {
                    pq.pop();
                }
                if pq.len() < k {
                    -1
                } else {
                    *pq.peek().unwrap()
                }
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
