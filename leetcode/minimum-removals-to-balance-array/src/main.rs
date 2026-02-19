// https://leetcode.com/problems/minimum-removals-to-balance-array/description/?envType=daily-question&envId=2026-02-06
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let k = k as i64;
        let n = nums.len();

        let mut to_remove_right = n;
        let mut right = 0;
        let mut min_ops = usize::MAX;

        for (to_remove_left, &num) in nums.iter().enumerate() {
            while right < n && nums[right] as i64 <= k * num as i64 {
                right += 1;
                to_remove_right -= 1;
            }

            min_ops = min_ops.min(to_remove_left + to_remove_right);
        }

        min_ops as i32
    }
}

fn main() {
    let test_cases = [
        (vec![2, 1, 5], 2, 1),
        (vec![1, 6, 2, 9], 3, 2),
        (
            vec![
                26155, 1776, 22815, 775, 27772, 12869, 12995, 22794, 27692, 24728, 10944, 25039,
                24068, 25506, 18506, 19138, 12331, 17814, 20834, 21474, 20208, 21590, 15453, 6114,
                25716, 29434, 23547, 29051, 25992, 5535, 7387,
            ],
            80020,
            0,
        ),
    ];

    for (nums, k, exp) in test_cases {
        println!("{} exp: {exp}", Solution::min_removal(nums, k));
    }
}
