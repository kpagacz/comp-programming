// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/description/
pub struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut first_decreasing = 1;
        let mut it = 1;
        while it < nums.len() && nums[it] >= nums[it - 1] {
            first_decreasing = it + 1;
            it += 1;
        }

        dbg!(first_decreasing);
        first_decreasing == nums.len()
            || (dbg!(nums[0] >= nums[nums.len() - 1])
                && dbg!(nums[first_decreasing..]
                    .windows(2)
                    .all(|window| window[0] <= window[1])))
    }
}

fn main() {
    let test_cases = [vec![2, 1]];
    for nums in test_cases {
        println!("{}", Solution::check(nums));
    }
}
