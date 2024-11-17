// https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/description/
pub struct Solution;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut pq = BinaryHeap::default();

        let mut answer = usize::MAX;
        let mut cum_sum = 0i64;

        for i in 0..nums.len() {
            cum_sum += nums[i] as i64;
            if cum_sum >= k as i64 {
                answer = answer.min(i + 1);
            }
            while let Some(Reverse((other_sum, pos))) = pq.peek() {
                if cum_sum - other_sum >= k as i64 {
                    answer = answer.min(i - pos);
                    pq.pop();
                } else {
                    break;
                }
            }

            pq.push(Reverse((cum_sum, i)));
        }

        if answer == usize::MAX {
            -1
        } else {
            answer as _
        }
    }
}

fn main() {
    let test_cases = [(vec![-28, 81, -20, 28, -29], 89)];
    for (nums, k) in test_cases {
        println!("{}", Solution::shortest_subarray(nums, k));
    }
}
