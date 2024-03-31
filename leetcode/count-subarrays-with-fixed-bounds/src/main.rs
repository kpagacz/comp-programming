// https://leetcode.com/problems/count-subarrays-with-fixed-bounds/description/
pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut answer = 0;
        let mut last_bad = -1;
        let mut last_min_k = -1;
        let mut last_max_k = -1;

        for (i, num) in nums.iter().enumerate() {
            if *num < min_k || *num > max_k {
                last_bad = i as i64;
            }

            if *num == min_k {
                last_min_k = i as i64;
            }

            if *num == max_k {
                last_max_k = i as i64;
            }

            answer += 0.max(last_min_k.min(last_max_k) - last_bad);
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
