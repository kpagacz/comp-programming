// https://leetcode.com/problems/majority-element-ii/description/
pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut frequencies = HashMap::new();

        for &num in &nums {
            frequencies
                .entry(num)
                .and_modify(|freq| *freq += 1)
                .or_insert(1);

            if frequencies.len() == 3 {
                frequencies.values_mut().for_each(|freq| *freq -= 1);
                frequencies.retain(|_, freq| *freq != 0);
            }
        }

        frequencies.retain(|key, _| nums.iter().filter(|&num| num == key).count() > nums.len() / 3);
        frequencies
            .iter()
            .map(|(&key, _)| key)
            .collect::<Vec<i32>>()
    }
}

fn main() {
    println!("Hello, world!");
}
