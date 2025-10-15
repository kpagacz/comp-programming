// https://leetcode.com/problems/adjacent-increasing-subarrays-detection-ii/description/?envType=daily-question&envId=2025-10-15
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut left = 1;
        let mut right = nums.len() / 2;

        while left < right {
            let mid = left + (right - left) / 2;

            if Solution::has_increasing_subarrays(&nums, mid) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if left == 1 { 1 } else { (left - 1) as _ }
    }

    pub fn max_increasing_subarrays_smarter(nums: Vec<i32>) -> i32 {
        let mut previous_count = 1;
        let mut count = 1;
        let mut answer = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                count += 1;
            } else {
                previous_count = count;
                count = 1;
            }
            answer = answer.max(previous_count.min(count));
            answer = answer.max(count / 2);
        }
        answer
    }

    fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
        if k == 1 {
            return true;
        }
        let mut slow_inversions = 0;
        let mut fast_inversions = 0;

        for i in 1..k {
            if nums[i] <= nums[i - 1] {
                slow_inversions += 1;
            }
            if nums[k + i] <= nums[k + i - 1] {
                fast_inversions += 1;
            }
        }

        if slow_inversions == 0 && fast_inversions == 0 {
            return true;
        }

        for i in 2 * k..nums.len() {
            if nums[i - 2 * k + 1] <= nums[i - 2 * k] {
                slow_inversions -= 1;
            }
            if nums[i - k] <= nums[i - k - 1] {
                slow_inversions += 1;
            }
            if nums[i - k + 1] <= nums[i - k] {
                fast_inversions -= 1;
            }
            if nums[i] <= nums[i - 1] {
                fast_inversions += 1;
            }
            if slow_inversions == 0 && fast_inversions == 0 {
                return true;
            }
        }

        false
    }
}

fn main() {
    println!("Hello, world!");
}
