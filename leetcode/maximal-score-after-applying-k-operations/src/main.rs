// https://leetcode.com/problems/maximal-score-after-applying-k-operations/description/
pub struct Solution;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;
        let mut max_heap = BinaryHeap::from(nums);
        let mut res = 0i64;
        for _ in 0..k {
            if max_heap.is_empty() {
                break;
            }
            let el = max_heap.pop().unwrap();
            res += el as i64;
            max_heap.push((el + 2) / 3);
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}
