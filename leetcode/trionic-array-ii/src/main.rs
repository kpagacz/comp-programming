// https://leetcode.com/problems/trionic-array-ii/description/?envType=daily-question&envId=2026-02-04
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let mut inversions = vec![0; nums.len()];
        for i in 2..nums.len() {
            if ((nums[i] - nums[i - 1]) as i64) * ((nums[i - 1] - nums[i - 2]) as i64) < 0i64 {
                inversions[i] = 1;
            }
        }
        let mut prefix_inversions = vec![0; inversions.len()];
        for i in 1..prefix_inversions.len() {
            prefix_inversions[i] = prefix_inversions[i - 1] + inversions[i];
        }

        let mut left = 0;
        let mut max_sum = i64::MIN;
        let mut current_sum = 0;
        for i in 0..nums.len() {
            current_sum += nums[i] as i64;

            // Equal case - need to reset the chain
            if i > 0 && nums[i - 1] == nums[i] {
                current_sum = nums[i] as i64;
                left = i;
                continue;
            }

            if i - left < 3 {
                continue;
            }

            let mut current_invs = prefix_inversions[i] - prefix_inversions[left + 1];

            while i - left >= 3 && (nums[left + 1] - nums[left] <= 0 || current_invs > 2) {
                current_sum -= nums[left] as i64;
                left += 1;
                current_invs = prefix_inversions[i] - prefix_inversions[left + 1];
            }

            if i - left >= 3 && nums[left + 1] - nums[left] > 0 && current_invs == 2 {
                // Move left if it's negative
                while i - left >= 4 && nums[left] < 0 && nums[left + 2] - nums[left + 1] > 0 {
                    current_sum -= nums[left] as i64;
                    left += 1;
                }
                max_sum = max_sum.max(current_sum);
            }
        }

        max_sum
    }
}

fn main() {
    let test_cases = [
        (vec![0, -2, -1, -3, 0, 2, -1], -4),
        (vec![1, 4, 2, 7], 14),
        (vec![1, 4, 2, 2, 3, 1, 2], 8),
        (vec![-754, 167, -172, 202, 735, -941, 992], 988),
        (vec![-895, 598, -819, -159, 52, -744, 977, -564, 774], 443),
        (vec![-35, 581, -864, -562, 671, -971, 571, 374], -209),
        (
            vec![
                -15414, -13280, -10121, -6588, -5728, -5041, -3636, -2228, -631, 2498, 4031, 7381,
                9984, 13533, 13828, 17431, 19185, 19779, 20785, 23058, 23264, 26050, 29077, 23743,
                20324, 19692, 17559, 15771, 15390, 12756, 12130, 9291, 6230, 4227, 1882, -1387,
                -2767, -3723, -7484, -10017, -11624, -13794, -15416, -18661, -22395, -22822,
                -23750, -24975, -28543, -28565, -28998, -32443, -33244, -34062, -35478, -38861,
                -38235, -35443, -33661, -32819, -30804, -26911, -26302, -23197, -20909, 30842,
                -10066, 34029, -36746, 31644, 47667, -247, -32634, -6630, -43822, 9867, -20947,
                31264, 17429, -20489, -42499, 30686, 48408, 17911, 16119, -21122, 49897, 45857,
                19230, -24581, -49219, -8993, 2374, 9678, -7248,
            ],
            141899,
        ),
    ];

    for (nums, exp) in test_cases {
        println!("{} exp {exp}", Solution::max_sum_trionic(nums));
    }
}
