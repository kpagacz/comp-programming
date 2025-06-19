// https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/description/?envType=daily-question&envId=2025-06-19
pub struct Solution;

impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let mut previous_min = nums[0];
        let mut answer = 0;
        for num in nums {
            if num - previous_min > k {
                answer += 1;
                previous_min = num;
            }
        }
        answer + 1
    }
}

fn main() {
    println!("Hello, world!");
}
