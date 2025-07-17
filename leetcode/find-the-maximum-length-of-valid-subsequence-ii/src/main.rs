// https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii/description/?envType=daily-question&envId=2025-07-17
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![0; k]; k];
        let mut res = 0;
        for num in nums {
            let mod_num = (num % k as i32) as usize;
            for prev in 0..k {
                dp[prev][mod_num] = dp[mod_num][prev] + 1;
                res = res.max(dp[prev][mod_num]);
            }
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}
