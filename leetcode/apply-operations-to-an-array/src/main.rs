// https://leetcode.com/problems/apply-operations-to-an-array/description/
pub struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut write_index = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
            if nums[i] != 0 {
                nums.swap(i, write_index);
                write_index += 1;
            }
        }
        if nums[nums.len() - 1] != 0 {
            let n = nums.len();
            nums.swap(n - 1, write_index);
        }
        nums
    }
}

fn main() {
    println!("Hello, world!");
}
