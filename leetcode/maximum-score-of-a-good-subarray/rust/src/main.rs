// https://leetcode.com/problems/maximum-score-of-a-good-subarray/description/
pub struct Solution {}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right) = (k, k);
        let must_include = k as usize;
        let mut maximum_score = i32::MIN;
        let mut curr_min = nums[must_include];
        let nums_count = nums.len() as i32;

        while left >= 0 || right < nums_count {
            while left >= 0 && nums[left as usize] >= curr_min {
                left -= 1;
            }
            while right < nums_count && nums[right as usize] >= curr_min {
                right += 1;
            }
            maximum_score = maximum_score.max(curr_min * (right - left - 1));
            curr_min = std::cmp::max(
                if left >= 0 { nums[left as usize] } else { i32::MIN },
                if right < nums_count { nums[right as usize] } else { i32::MIN },
            )
        }

        maximum_score
    }
    pub fn maximum_score_greedy_but_slow(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut maximum_score = i32::MIN;

        let mut right = k;
        let mut curr_min = nums[k];
        ((0..=k).rev()).for_each(|it| {
            curr_min = curr_min.min(nums[it]);
            while right + 1 < nums.len() && nums[right + 1] >= curr_min {
                right += 1;
            }
            maximum_score = maximum_score.max((right - it + 1) as i32 * curr_min);
        });

        let mut left = k;
        curr_min = nums[k];
        (k..nums.len()).for_each(|it| {
            curr_min = curr_min.min(nums[it]);
            while left > 0 && nums[left - 1] >= curr_min {
                left -= 1;
            }
            maximum_score = maximum_score.max(curr_min * (it - left + 1) as i32);
        });
        maximum_score
    }
}

fn main() {
    println!("Hello, world!");
}
