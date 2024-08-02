// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/description/
pub struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let ones = nums.iter().filter(|&&num| num == 1).count();
        let mut current_ones = nums.iter().take(ones).filter(|&&num| num == 1).count();
        let mut max_ones = current_ones;

        for i in ones..nums.len() + ones {
            if nums[i - ones] == 1 {
                current_ones -= 1;
            }
            if nums[i % nums.len()] == 1 {
                current_ones += 1;
            }

            max_ones = usize::max(max_ones, current_ones);
        }

        ones as i32 - max_ones as i32
    }
}

fn main() {
    println!("Hello, world!");
}
