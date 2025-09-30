// https://leetcode.com/problems/find-triangular-sum-of-an-array/?envType=daily-question&envId=2025-09-30
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        while nums.len() > 1 {
            let mut new_nums = Vec::with_capacity(nums.len() - 1);
            for window in nums.windows(2) {
                let value = (window[0] + window[1]) % 10;
                new_nums.push(value);
            }
            nums = new_nums;
        }

        nums[0]
    }
}

fn main() {
    println!("Hello, world!");
}
