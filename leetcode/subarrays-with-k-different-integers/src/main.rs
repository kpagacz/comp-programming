// https://leetcode.com/problems/subarrays-with-k-different-integers/description/
pub struct Solution;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        (Solution::subarrays_with_at_least_k_distinct(&nums, k)
            - Solution::subarrays_with_at_least_k_distinct(&nums, k + 1)) as _
    }

    fn subarrays_with_at_least_k_distinct(nums: &[i32], k: i32) -> i64 {
        let mut left = 0;
        let mut answer = 0;
        let mut freqs = vec![0; nums.len() + 1];
        let mut distinct = 0;

        for (i, num) in nums.iter().enumerate() {
            if freqs[*num as usize] == 0 {
                distinct += 1;
            }
            freqs[*num as usize] += 1;

            while left <= i && distinct >= k {
                answer += nums.len() - i;
                freqs[nums[left] as usize] -= 1;
                if freqs[nums[left] as usize] == 0 {
                    distinct -= 1;
                }
                left += 1;
            }
        }

        answer as _
    }
}

fn main() {
    let tests = [(vec![1, 2, 1, 2, 3], 2)];

    for (nums, k) in tests {
        println!("{}", Solution::subarrays_with_k_distinct(nums, k));
    }
}
