// https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array/description/?envType=daily-question&envId=2025-04-17
pub struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut indices = [const { Vec::new() }; 101];
        let mut answer = 0;

        let k = k as usize;
        for i in 0..nums.len() {
            let num = nums[i] as usize;
            answer += indices[num]
                .iter()
                .filter(|&&index| (index * i) % k == 0)
                .count();
            indices[num].push(i);
        }

        answer as _
    }
}

fn main() {
    println!("Hello, world!");
}
