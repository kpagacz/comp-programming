// https://leetcode.com/problems/maximum-swap/description/
pub struct Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        use std::collections::BTreeMap;
        let mut digits: BTreeMap<char, Vec<_>> = BTreeMap::default();

        for (pos, c) in num.to_string().chars().enumerate() {
            digits.entry(c).or_default().push(pos);
        }

        let mut max_num: Vec<_> = num.to_string().chars().collect();
        for (pos, c) in num.to_string().chars().enumerate() {
            if let Some((_, positions)) = digits.range((((c as u8) + 1) as char)..).last() {
                max_num.swap(pos, *positions.iter().max().unwrap());
                return max_num.iter().collect::<String>().parse::<i32>().unwrap();
            }

            let c_entry = digits.entry(c).or_default();
            c_entry.retain(|saved_pos| *saved_pos != pos);
            if c_entry.is_empty() {
                digits.remove(&c);
            }
        }

        num
    }
}

fn main() {
    let test_cases = [2736, 9932, 9392];
    for num in test_cases {
        println!("{}", Solution::maximum_swap(num));
    }
}
