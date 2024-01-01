// https://leetcode.com/problems/assign-cookies/
pub struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();
        let mut answer = 0;
        let (mut i, mut j) = (0, 0);
        while i < g.len() && j < s.len() {
            if g[i] <= s[j] {
                i += 1;
                j += 1;
                answer += 1;
            } else {
                j += 1;
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
