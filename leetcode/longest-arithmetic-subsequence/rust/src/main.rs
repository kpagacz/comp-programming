// https://leetcode.com/problems/longest-arithmetic-subsequence/description/

pub struct Solution {}
impl Solution {
    pub fn longest_arith_seq_length_tle(nums: Vec<i32>) -> i32 {
        let mut longest_arithmetic_seq = 0;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let mut arith_seq_length = 2;
                let diff = nums[j] - nums[i];
                let mut last = nums[j];
                for k in (j + 1)..nums.len() {
                    if nums[k] - last == diff {
                        arith_seq_length += 1;
                        last = nums[k];
                    }
                }
                longest_arithmetic_seq = std::cmp::max(longest_arithmetic_seq, arith_seq_length);
            }
        }
        longest_arithmetic_seq
    }
    pub fn longest_arith_seq_length_passes_but_slow(nums: Vec<i32>) -> i32 {
        let mut mapped_numbers = vec![vec![-1; 501]; nums.len()];
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if mapped_numbers[i][nums[j] as usize] == -1 {
                    mapped_numbers[i][nums[j] as usize] = j as i32;
                }
            }
        }

        let mut max_length = 0;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let diff = nums[j] - nums[i];
                let mut length = 2;
                let mut k = j;
                while (nums[k] + diff) >= 0
                    && ((nums[k] + diff) as usize) <= 500
                    && mapped_numbers[k][(nums[k] + diff) as usize] != -1
                {
                    k = mapped_numbers[k][(nums[k] + diff) as usize] as usize;
                    length += 1;
                }
                max_length = std::cmp::max(max_length, length);
            }
        }

        max_length
    }

    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let &max = nums.iter().max().unwrap();
        (-max..=max)
            .map(|step| {
                let mut dp = vec![0; max as usize + 1];
                for &num in &nums {
                    dp[num as usize] =
                        dp[num as usize].max(dp.get((num - step) as usize).unwrap_or(&0) + 1);
                }
                *dp.iter().max().unwrap()
            })
            .max()
            .unwrap()
    }
}

fn main() {
    let tests = vec![
        vec![3, 6, 9, 12],
        vec![9, 4, 7, 2, 10],
        vec![20, 1, 15, 3, 10, 5, 8],
    ];
    for test in tests {
        println!("{}", Solution::longest_arith_seq_length(test));
    }
}
