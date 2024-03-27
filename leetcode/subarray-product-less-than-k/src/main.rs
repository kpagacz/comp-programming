// https://leetcode.com/problems/subarray-product-less-than-k/description/
pub struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut answer = 0;
        let (mut left, mut right) = (0, 0);
        let mut current = nums[0];

        while left < nums.len() {
            if left >= right + 2 {
                right = left - 1;
            }
            while right + 1 < nums.len() && current * nums[right + 1] < k {
                right += 1;
                current *= nums[right];
            }

            if current < k {
                answer += right - left + 1;
            }

            current /= nums[left];
            current = current.max(1);
            left += 1;
        }

        answer as _
    }

    pub fn smarter(nums: Vec<i32>, k: i32) -> i32 {
        let mut answer = 0;
        let mut left = 0;
        let mut current = 1;

        for right in 0..nums.len() {
            current *= nums[right];
            while current >= k && left <= right {
                current /= nums[left];
                left += 1;
            }
            answer += right - left + 1;
        }

        answer as _
    }
}

fn main() {
    let tests = [([10, 5, 2, 6], 100)];

    for (nums, k) in tests {
        println!(
            "{}",
            Solution::num_subarray_product_less_than_k(nums.to_vec(), k)
        );
    }
}
