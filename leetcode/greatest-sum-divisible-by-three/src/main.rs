// https://leetcode.com/problems/greatest-sum-divisible-by-three/description/?envType=daily-question&envId=2025-11-23
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut ones = Vec::with_capacity(nums.len());
        let mut twos = Vec::with_capacity(nums.len());
        let mut answer: i32 = 0;
        for num in nums {
            match num % 3 {
                0 => answer += num,
                1 => ones.push(num),
                2 => twos.push(num),
                _ => unreachable!(),
            }
        }
        ones.sort_unstable();
        twos.sort_unstable();
        while ones.len() > 5 {
            for _ in 0..3 {
                answer += ones.pop().unwrap();
            }
        }
        while twos.len() > 5 {
            for _ in 0..3 {
                answer += twos.pop().unwrap()
            }
        }
        let mut dp = vec![vec![0; twos.len() + 1]; ones.len() + 1];
        for taken_ones in 0..dp.len() {
            for taken_twos in 0..dp[0].len() {
                let mut max = dp[taken_ones][taken_twos];
                // Skip
                if taken_ones > 0 {
                    max = max.max(dp[taken_ones - 1][taken_twos]);
                }
                if taken_twos > 0 {
                    max = max.max(dp[taken_ones][taken_twos - 1]);
                }

                // Take 3 ones
                if taken_ones >= 3 {
                    max = max.max(
                        dp[taken_ones - 3][taken_twos]
                            + ones[taken_ones - 3..taken_ones].iter().sum::<i32>(),
                    );
                }
                // Take 3 twos
                if taken_twos >= 3 {
                    max = max.max(
                        dp[taken_ones][taken_twos - 3]
                            + twos[taken_twos - 3..taken_twos].iter().sum::<i32>(),
                    );
                }
                // Take 1 one and 1 two
                if taken_ones > 0 && taken_twos > 0 {
                    max = max.max(
                        dp[taken_ones - 1][taken_twos - 1]
                            + ones[taken_ones - 1]
                            + twos[taken_twos - 1],
                    );
                }

                dp[taken_ones][taken_twos] = max;
            }
        }
        answer + dp[ones.len()][twos.len()]
    }
}

fn main() {
    let test_cases = [
        (vec![3, 6, 5, 1, 8], 18),
        (vec![4], 0),
        (vec![1, 2, 3, 4, 4], 12),
        (vec![1, 1, 1, 1, 1, 1, 2, 2, 2], 12),
        (vec![2, 19, 6, 16, 5, 10, 7, 4, 11, 6], 84),
    ];
    for (nums, exp) in test_cases {
        println!("{}  exp: {exp}", Solution::max_sum_div_three(nums));
    }
}
