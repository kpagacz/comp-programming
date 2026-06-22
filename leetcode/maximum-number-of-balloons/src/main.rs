// https://leetcode.com/problems/maximum-number-of-balloons/description/?envType=daily-question&envId=2026-06-22
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        use std::collections::HashMap;
        let mut chars = HashMap::new();
        for c in text.chars() {
            chars.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut times = i32::MAX;
        for single_occ_letter in ['b', 'a', 'n'] {
            times = times.min(*chars.get(&single_occ_letter).unwrap_or(&0));
        }
        for double_occ_letter in ['l', 'o'] {
            times = times.min(*chars.get(&double_occ_letter).unwrap_or(&0) / 2);
        }
        times
    }
}

fn main() {
    println!("Hello, world!");
}
