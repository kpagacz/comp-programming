// https://leetcode.com/problems/kth-largest-element-in-a-stream/description/

use std::collections::BinaryHeap;
struct KthLargest {
    nums: BinaryHeap<i32>,
    k: usize,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::from_iter(nums.into_iter().map(|num| -num));
        while heap.len() > k as usize {
            heap.pop();
        }
        Self {
            nums: heap,
            k: k as usize,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(-val);
        while self.nums.len() > self.k {
            self.nums.pop();
        }
        -*self.nums.peek().unwrap()
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

fn main() {
    println!("Hello, world!");
}
