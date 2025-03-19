// https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i/solutions/6553332/simple-greedy-python-c-java-js-c-go-swift-rust-kotlin-php/?envType=daily-question&envId=2025-03-19
pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;

        for i in 0..nums.len() - 2 {
            if nums[i] == 0 {
                nums[i] ^= 1;
                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;
                count += 1;
            }
        }

        let n = nums.len();
        if nums[n - 2] == 1 && nums[n - 1] == 1 {
            count
        } else {
            -1
        }
    }
}

fn main() {
    println!("Hello, world!");
}
