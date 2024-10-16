// https://leetcode.com/problems/longest-happy-string/description/
pub struct Solution;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut longest = Vec::with_capacity(a as usize + b as usize + c as usize);
        use std::collections::BinaryHeap;
        let mut stack = BinaryHeap::default();
        if a != 0 {
            stack.push((a, 'a'));
        }
        if b != 0 {
            stack.push((b, 'b'));
        }
        if c != 0 {
            stack.push((c, 'c'));
        }

        let are_last_two_same = |longest: &[char], c: char| {
            if longest.len() < 2 {
                false
            } else {
                longest[longest.len() - 1] == c && longest[longest.len() - 2] == c
            }
        };

        while let Some((count, c)) = stack.pop() {
            if are_last_two_same(&longest, c) {
                if let Some((next_count, next_c)) = stack.pop() {
                    longest.push(next_c);
                    if next_count - 1 != 0 {
                        stack.push((next_count - 1, next_c));
                    }
                    stack.push((count, c));
                } else {
                    break;
                }
            } else {
                longest.push(c);
                if count - 1 != 0 {
                    stack.push((count - 1, c));
                }
            }
        }

        longest.into_iter().collect::<String>()
    }
}

fn main() {
    let test_cases = [(1, 1, 7), (3, 3, 9)];
    for (a, b, c) in test_cases {
        println!("{}", Solution::longest_diverse_string(a, b, c));
    }
}
