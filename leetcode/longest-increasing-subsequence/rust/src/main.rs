// https://leetcode.com/problems/longest-increasing-subsequence/description/
pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        let n = dp.len();
        for i in 0..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        dp.into_iter().max().unwrap()
    }

    pub fn length_of_lis_smarter(nums: Vec<i32>) -> i32 {
        let mut arr = Vec::with_capacity(nums.len());
        arr.push(nums[0]);

        for &number in nums.iter().skip(1) {
            match number.cmp(arr.last().unwrap()) {
                std::cmp::Ordering::Less => match arr.binary_search(&number) {
                    Ok(_) => {}
                    Err(idx) => arr[idx] = number,
                },
                std::cmp::Ordering::Equal => continue,
                std::cmp::Ordering::Greater => arr.push(number),
            }
        }

        arr.len() as i32
    }
}

fn main() {
    println!("Hello, world!");
}
