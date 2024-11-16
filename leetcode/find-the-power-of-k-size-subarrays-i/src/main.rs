// https://leetcode.com/problems/find-the-power-of-k-size-subarrays-i/description/
pub struct Solution;

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut answer = vec![];
        let mut first_sorted_index = 0;

        for pos in 1..k {
            if nums[pos] != nums[pos - 1] + 1 {
                first_sorted_index = pos;
            }
        }
        if first_sorted_index == 0 {
            answer.push(nums[k - 1]);
        } else {
            answer.push(-1);
        }

        for pos in k..nums.len() {
            if nums[pos] != nums[pos - 1] + 1 {
                first_sorted_index = pos;
            }
            if first_sorted_index <= pos - k + 1 {
                answer.push(nums[pos]);
            } else {
                answer.push(-1);
            }
        }

        answer
    }
}

fn main() {
    let test_cases = [
        (vec![1, 2, 3, 4, 3, 2, 5], 3),
        (vec![2, 2, 2, 2, 2], 4),
        (vec![3, 2, 3, 2, 3, 2], 2),
        (vec![1], 1),
    ];
    for (nums, k) in test_cases {
        println!("{:?}", Solution::results_array(nums, k));
    }
}
