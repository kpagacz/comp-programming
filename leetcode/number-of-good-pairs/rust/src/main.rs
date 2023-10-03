// https://leetcode.com/problems/number-of-good-pairs/description/
pub struct Solution {}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut seen = vec![0; 101];
        let mut answer = 0;
        for num in nums {
            answer += seen[num as usize];
            seen[num as usize] += 1;
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
