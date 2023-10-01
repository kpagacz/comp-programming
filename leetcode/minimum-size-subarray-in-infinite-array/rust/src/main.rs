// https://leetcode.com/problems/minimum-size-subarray-in-infinite-array/description/
pub struct Solution {}
impl Solution {
    pub fn min_size_subarray(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        // The number of elements associated with fitting the whole array
        // in the final answer.
        let whole_arrays_in_answer = (target / sum) * nums.len() as i32;

        let normal = Solution::optimized_size_subarray(&nums, target % sum, i32::MAX, i32::min);
        let crossing =
            Solution::optimized_size_subarray(&nums, sum - (target % sum), i32::MIN, i32::max);

        match (normal == -1, crossing == -1) {
            (true, true) => return -1,
            (false, false) => {
                return whole_arrays_in_answer + normal.min(nums.len() as i32 - crossing)
            }
            (true, false) => return whole_arrays_in_answer + nums.len() as i32 - crossing,
            (false, true) => return whole_arrays_in_answer + normal,
        }
    }

    fn optimized_size_subarray(
        nums: &Vec<i32>,
        target: i32,
        start_value: i32,
        optimization_fn: fn(i32, i32) -> i32,
    ) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut current = 0;
        let mut optimized = start_value;
        while right < nums.len() {
            current += nums[right];

            while left <= right && current > target {
                current -= nums[left];
                left += 1;
            }
            if current == target {
                optimized = optimization_fn(optimized, (right - left) as i32 + 1);
            }

            right += 1;
        }

        if optimized == start_value {
            -1
        } else {
            optimized
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_size_subarray(
            vec![1, 4, 8, 5, 9, 8, 8, 2, 3, 1, 6, 2, 7, 5, 5, 3, 3, 5, 6],
            57
        )
    );
}
