// https://leetcode.com/problems/remove-duplicate-letters
pub struct Solution {}

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        use std::collections::HashMap;
        let mut char_counts = HashMap::new();

        for c in s.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }
        let mut monotonic_stack = Vec::new();

        let mut already_in = vec![false; 257];
        for c in s.chars() {
            while !monotonic_stack.is_empty()
                && char_counts[&monotonic_stack.last().unwrap()] != 1
                && c <= *monotonic_stack.last().unwrap()
                && already_in[c as usize] == false
            {
                *char_counts
                    .get_mut(&monotonic_stack.last().unwrap())
                    .unwrap() -= 1;
                already_in[*monotonic_stack.last().unwrap() as usize] = false;
                monotonic_stack.pop();
            }

            if !already_in[c as usize] {
                monotonic_stack.push(c);
                already_in[c as usize] = true;
            } else {
                *char_counts.get_mut(&c).unwrap() -= 1;
            }
        }

        monotonic_stack.into_iter().collect()
    }
}
fn main() {
    println!("{}", Solution::remove_duplicate_letters("bcabc".to_owned()));
    println!(
        "{}",
        Solution::remove_duplicate_letters("cbacdcbc".to_owned())
    );
}
