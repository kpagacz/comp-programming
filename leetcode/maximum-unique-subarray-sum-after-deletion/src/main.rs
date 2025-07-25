// https://leetcode.com/problems/maximum-unique-subarray-sum-after-deletion/description/?envType=daily-question&envId=2025-07-25
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.dedup();
        let largest = nums[nums.len() - 1];
        if nums.iter().all(|num| num < &0) {
            largest
        } else {
            nums.into_iter().filter(|&num| num > 0).sum()
        }
    }
}

fn main() {
    println!("Hello, world!");
}
