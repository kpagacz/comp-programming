// https://leetcode.com/problems/take-gifts-from-the-richest-pile/description/
pub struct Solution;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::from(gifts);
        for _ in 0..k {
            let top = heap.pop().unwrap();
            let sq = (top as f64).sqrt() as i32;
            heap.push(sq);
        }
        heap.into_iter().map(|num| num as i64).sum::<i64>()
    }
}

fn main() {
    println!("Hello, world!");
}
