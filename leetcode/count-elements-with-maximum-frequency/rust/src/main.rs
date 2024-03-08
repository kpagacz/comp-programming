// https://leetcode.com/problems/a/description/
pub struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        use std::collections::BTreeMap;
        let nums = nums.into_iter().fold(BTreeMap::new(), |mut map, num| {
            *map.entry(num).or_insert(0) += 1; 
            map
        });
        let max_freq = nums.values().max().unwrap();
        nums.values().fold(0, |acc, freq| if freq == max_freq { acc + freq } else { acc })
    }
}

fn main() {
    let tests = vec![vec![1, 2, 2, 3, 1, 4], vec![1, 2, 3, 4, 5]];
    for test in tests {
        println!("{}", Solution::max_frequency_elements(test));
    }
}
