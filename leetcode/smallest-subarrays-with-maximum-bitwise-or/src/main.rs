// https://leetcode.com/problems/smallest-subarrays-with-maximum-bitwise-or/description/?envType=daily-question&envId=2025-07-29
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let mut one_positions = [0; 32];

        let mut answer = vec![0; nums.len()];

        for pos in (0..nums.len()).rev() {
            let mut max_position = pos;
            (0..32).for_each(|i| {
                if ((1 << i) & nums[pos]) == 0 {
                    max_position = max_position.max(one_positions[i]);
                } else {
                    one_positions[i] = pos;
                }
            });
            answer[pos] = (max_position - pos + 1) as _;
        }
        answer
    }
}

fn main() {
    let test_cases = [vec![1, 0, 2, 1, 3]];

    for nums in test_cases {
        println!("{:?}", Solution::smallest_subarrays(nums));
    }
}
