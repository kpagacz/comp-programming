// https://leetcode.com/problems/continuous-subarrays/description/
pub struct Solution;

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        use std::collections::BTreeMap;
        let mut seen = BTreeMap::default();

        let mut num = 0;
        let mut left = 0;
        for i in 0..nums.len() {
            *seen.entry(nums[i]).or_insert(0) += 1;

            let mut first = seen.first_key_value().unwrap().0;
            let mut second = seen.last_key_value().unwrap().0;
            while first.abs_diff(*second) > 2 && left < i {
                let entry = seen.entry(nums[left]).or_insert(0);
                *entry -= 1;
                if *entry == 0 {
                    seen.remove(&nums[left]);
                }
                first = seen.first_key_value().unwrap().0;
                second = seen.last_key_value().unwrap().0;
                left += 1;
            }

            num += i as i64 - left as i64 + 1;
        }
        num
    }
}

fn main() {
    println!("Hello, world!");
}
