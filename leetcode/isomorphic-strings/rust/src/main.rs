// https://leetcode.com/problems/isomorphic-strings/
pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let mut already_mapped = HashSet::new();
        let mut s_to_t = HashMap::new();

        let s = s.as_bytes();
        let t = t.as_bytes();

        if s.len() != t.len() {
            return false;
        }

        for i in 0..s.len() {
            if let Some(dst) = s_to_t.get(&s[i]) {
                if *dst != t[i] {
                    return false;
                }
            } else {
                // s[i] not mapped yet
                if already_mapped.contains(&t[i]) {
                    return false;
                }
                already_mapped.insert(t[i]);
                s_to_t.insert(s[i], t[i]);
            }
        }

        true
    }

    fn clever(s: String, t: String) -> bool {
        use std::collections::HashMap;

        let mut mapped_s = HashMap::new();
        let mut mapped_t = HashMap::new();

        let s = s.as_bytes();
        let t = t.as_bytes();

        for (s_char, t_char) in s.iter().zip(t.iter()) {
            match (mapped_s.get(s_char), mapped_t.get(t_char)) {
                (Some(&s_to_t), Some(&t_to_s)) => {
                    if t_to_s != s_char || s_to_t != t_char {
                        return false;
                    }
                }
                (None, None) => {
                    mapped_s.insert(s_char, t_char);
                    mapped_t.insert(t_char, s_char);
                }
                _ => return false,
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
