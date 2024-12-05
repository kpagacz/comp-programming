// https://leetcode.com/problems/move-pieces-to-obtain-a-string/description/
pub struct Solution;

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let s = start.as_bytes();
        let t = target.as_bytes();
        let mut sit = 0;
        let mut tit = 0;

        if start.len() != target.len() {
            return false;
        }
        loop {
            while sit < s.len() && s[sit] == b'_' {
                sit += 1;
            }
            while tit < s.len() && t[tit] == b'_' {
                tit += 1;
            }
            if sit == s.len() || tit == t.len() {
                break;
            }

            // Different chars
            if t[tit] != s[sit] {
                return false;
            }
            // It's an L and the target is to the right
            if s[sit] == b'L' && tit > sit {
                return false;
            }
            // It's an R and the target is to the left
            if s[sit] == b'R' && tit < sit {
                return false;
            }
            sit += 1;
            tit += 1;
        }

        sit == tit
    }
}

fn main() {
    println!("Hello, world!");
}
