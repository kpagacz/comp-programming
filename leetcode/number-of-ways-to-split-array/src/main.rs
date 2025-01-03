// https://leetcode.com/problems/number-of-ways-to-split-array/description/
pub struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut total = nums.iter().map(|num| *num as i64).sum::<i64>();
        let mut current = 0i64;
        let mut ans = 0i32;
        for &num in &nums[..nums.len() - 1] {
            current += num as i64;
            total -= num as i64;
            if current >= total {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
