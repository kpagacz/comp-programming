// https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/description/?envType=daily-question&envId=2025-06-29
pub struct Solution;

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let mut right = nums.len() - 1;
        let mut answer = 0;
        for left in 0..nums.len() {
            while nums[left] + nums[right] > target && right > left {
                right -= 1;
            }
            if nums[left] + nums[right] <= target {
                // Sum of finite geometric series is
                // (a * (r^n - 1)) / r - 1
                // where a is the first element
                // r is the common ratio
                // n is the number of elements
                // In this case we have:
                // left ... right elements.
                // So the sum is:
                // 1 + 2^1 + 2^2 + ... + 2^k
                // where k is (right - left - 1).min(0)
                // So there is k + 1 elements, which
                // is right - left elements.
                // So the sum is:
                // (1 * (2 ^ (right - left) - 1)) / 2 - 1 =
                // 2 ^ (right - left) - 1
                const MOD: i32 = 10i32.pow(9) + 7;
                answer += Self::fast_pow_with_modulo(2, right - left, MOD as i64) as i32;
                answer %= MOD;
            }
        }
        answer
    }

    fn fast_pow_with_modulo(mut b: i64, mut exp: usize, m: i64) -> i64 {
        let mut answer = 1;

        while exp > 0 {
            if exp & 1 == 1 {
                answer *= b;
                answer %= m;
                exp -= 1;
            }
            b *= b;
            b %= m;
            exp /= 2;
        }

        answer
    }
}

fn main() {
    let test_cases = [(vec![2, 3, 3, 4, 6, 7], 12)];
    for (nums, target) in test_cases {
        println!("{}", Solution::num_subseq(nums, target));
    }
}
