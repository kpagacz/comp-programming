// https://leetcode.com/problems/adding-spaces-to-a-string/description/
pub struct Solution;

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut res = String::with_capacity(s.len() + spaces.len());
        let mut last = 0;
        for space in spaces {
            let space = space as usize;
            res.push_str(&s[last..space]);
            res.push(' ');
            last = space;
        }
        res.push_str(&s[last..]);
        res
    }
}

fn main() {
    println!("Hello, world!");
}
