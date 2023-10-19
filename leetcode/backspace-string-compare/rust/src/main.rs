// https://leetcode.com/problems/backspace-string-compare/
pub struct Solution {}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let hash = '#' as u8;
        let s = s.bytes().collect::<Vec<u8>>();
        let t = t.bytes().collect::<Vec<u8>>();

        let next_char = |mut start: usize, array: &Vec<u8>| -> usize {
            if start == 0 {
                return usize::MAX;
            }
            let mut hashes = 1;
            while hashes > 0 {
                if start == 0 {
                    return usize::MAX;
                }
                start -= 1;
                if array[start] == hash {
                    hashes += 1;
                } else {
                    hashes -= 1;
                }
            }

            start
        };

        let mut s_char = s.len();
        let mut t_char = t.len();
        loop {
            s_char = next_char(s_char, &s);
            t_char = next_char(t_char, &t);
            match (s_char == usize::MAX, t_char == usize::MAX) {
                (true, true) => return true,
                (false, true) => return false,
                (true, false) => return false,
                (false, false) => {
                    if s[s_char] != t[t_char] {
                        return false;
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
