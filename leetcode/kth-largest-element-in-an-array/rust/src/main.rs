// https://leetcode.com/problems/kth-largest-element-in-an-array/description/
pub struct Solution {}

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        *nums
            .select_nth_unstable_by((k - 1) as usize, |a, b| b.cmp(a))
            .1
    }

    pub fn heap_find(nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut maxHeap = BinaryHeap::with_capacity(k as usize + 1);

        for num in nums {
            maxHeap.push(Reverse(num));

            if maxHeap.len() > k as usize {
                maxHeap.pop();
            }
        }

        maxHeap.peek().unwrap().0
    }
}

fn main() {
    println!("Hello, world!");
}
