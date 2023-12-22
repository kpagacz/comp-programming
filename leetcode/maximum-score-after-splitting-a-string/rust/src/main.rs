// https://leetcode.com/problems/maximum-score-after-splitting-a-string/description/
pub struct Solution {}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut ones = s.iter().filter(|&&s| s == '1').count();
        let mut zeros = 0;

        let mut ans = 0;
        for &c in &s[..s.len() - 1] {
            if c == '0' {
                zeros += 1;
            } else {
                ones -= 1;
            }
            ans = ans.max(ones + zeros);
        }

        ans as i32
    }
}

fn main() {
    println!("Hello, world!");
}
