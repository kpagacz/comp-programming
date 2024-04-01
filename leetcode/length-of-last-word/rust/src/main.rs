// https://leetcode.com/problems/length-of-last-word/description/
pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let trimmed = s.trim_end();
        match trimmed.rsplit_once(' ') {
            Some((_, suffix)) => suffix.len() as _,
            None => trimmed.len() as _,
        }
    }

    fn another_way(s: String) -> i32 {
        s.split_whitespace().next_back().unwrap().len() as _
    }
}

fn main() {
    let s = " luffy    ".to_owned();
    println!("{}", Solution::length_of_last_word(s.clone()));
    println!("{}", Solution::another_way(s.clone()));
}
