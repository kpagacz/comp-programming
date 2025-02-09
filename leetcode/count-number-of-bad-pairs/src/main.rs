// https://leetcode.com/problems/count-number-of-bad-pairs/description
pub struct Solution;

impl Solution {
    pub fn count_bad_pairs(mut nums: Vec<i32>) -> i64 {
        use std::collections::HashMap;
        nums.iter_mut()
            .enumerate()
            .for_each(|(pos, num)| *num = pos as i32 - *num);
        let mut freqs = HashMap::<i32, i64>::new();
        let mut total = 0;
        for (pos, num) in nums.into_iter().enumerate() {
            let seen = *freqs.get(&num).unwrap_or(&0);
            total += (pos as i64 - seen + 1) * (pos as i64 - seen) / 2;
            *freqs.entry(num).or_default() += 1;
        }
        total
    }
}

fn main() {
    println!("Hello, world!");
}
