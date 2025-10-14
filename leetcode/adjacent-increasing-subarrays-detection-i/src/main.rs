// https://leetcode.com/problems/adjacent-increasing-subarrays-detection-i/description/?envType=daily-question&envId=2025-10-14
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;

        let mut fast_inversions = 0;
        let mut slow_inversions = 0;

        for pair in nums.windows(2).take(k - 1) {
            if pair[1] <= pair[0] {
                slow_inversions += 1;
            }
        }
        for pair in nums.windows(2).skip(k).take(k - 1) {
            if pair[1] <= pair[0] {
                fast_inversions += 1;
            }
        }

        if fast_inversions == 0 && slow_inversions == 0 {
            return true;
        }

        for i in 2 * k..nums.len() {
            if nums[i + 1 - 2 * k] <= nums[i - 2 * k] {
                slow_inversions -= 1;
            }
            if nums[i - k] <= nums[i - k - 1] {
                slow_inversions += 1;
            }
            if nums[i + 1 - k] <= nums[i - k] {
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
    let test_cases = [
        (vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1], 3),
        (vec![-15, 3, 16, 0], 2),
    ];

    for (nums, k) in test_cases {
        println!("{}", Solution::has_increasing_subarrays(nums, k));
    }
}
