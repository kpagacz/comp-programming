// https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/description/
pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        use std::collections::BTreeMap;
        let mut numbers = BTreeMap::new();
        let mut max_length = 0;
        let mut left = 0;

        for i in 0..nums.len() {
            *numbers.entry(nums[i]).or_insert(0) += 1;

            let mut first_value = *numbers.first_key_value().unwrap().0;
            let mut last_value = *numbers.last_key_value().unwrap().0;
            while last_value - first_value > limit {
                *numbers.entry(nums[left]).or_insert(0) -= 1;
                if numbers.get(&nums[left]).unwrap() == &0 {
                    numbers.remove(&nums[left]);
                }

                first_value = *numbers.first_key_value().unwrap().0;
                last_value = *numbers.last_key_value().unwrap().0;
                left += 1;
            }

            max_length = max_length.max(i - left + 1);
        }

        max_length as _
    }

    pub fn longest_subarray_with_dequeues(nums: Vec<i32>, limit: i32) -> i32 {
        use std::collections::VecDeque;

        let mut answer = 0;
        let mut min_queue = VecDeque::new();
        let mut max_queue = VecDeque::new();
        let mut left = 0;

        for i in 0..nums.len() {
            while !min_queue.is_empty() && nums[i] < nums[*min_queue.back().unwrap()] {
                min_queue.pop_back();
            }
            while !max_queue.is_empty() && nums[i] > nums[*max_queue.back().unwrap()] {
                max_queue.pop_back();
            }
            min_queue.push_back(i);
            max_queue.push_back(i);

            while nums[*max_queue.front().unwrap()] - nums[*min_queue.front().unwrap()] > limit {
                if max_queue.front().unwrap() == &left {
                    max_queue.pop_front();
                }
                if min_queue.front().unwrap() == &left {
                    min_queue.pop_front();
                }
                left += 1;
            }

            answer = answer.max(i - left + 1);
        }

        answer as _
    }
}

fn main() {
    println!("Hello, world!");
}
