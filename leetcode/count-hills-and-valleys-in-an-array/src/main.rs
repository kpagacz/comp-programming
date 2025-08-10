// https://leetcode.com/problems/count-hills-and-valleys-in-an-array/submissions/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_hill_valley(mut nums: Vec<i32>) -> i32 {
        nums.dedup();

        let is_hill = |i| nums[i] > nums[i - 1] && nums[i] > nums[i + 1];
        let is_valley = |i| nums[i] < nums[i - 1] && nums[i] < nums[i + 1];

        (1..nums.len() - 1)
            .filter(|&i| is_hill(i) || is_valley(i))
            .count() as _
    }
}

fn main() {
    println!("Hello, world!");
}
