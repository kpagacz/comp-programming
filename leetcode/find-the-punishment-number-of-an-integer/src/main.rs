// https://leetcode.com/problems/find-the-punishment-number-of-an-integer/description/
pub struct Solution;

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        (1..=n)
            .filter_map(|num| {
                if Self::is_punishment(num * num, num) {
                    Some(num * num)
                } else {
                    None
                }
            })
            .sum::<i32>()
    }

    fn is_punishment(split: i32, target: i32) -> bool {
        if target < 0 || split < target {
            false
        } else if split == target {
            true
        } else {
            Self::is_punishment(split / 10, target - split % 10)
                || Self::is_punishment(split / 100, target - split % 100)
                || Self::is_punishment(split / 1000, target - split % 1000)
        }
    }
}

fn main() {
    println!("Hello, world!");
}
