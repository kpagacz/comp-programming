// https://leetcode.com/problems/make-the-string-great/description/
pub struct Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut s: Vec<_> = s.chars().collect();

        loop {
            let mut is_string_bad = false;
            let mut it = 0;
            while it < s.len() - 1 {
                if s[it].to_lowercase().next().unwrap() == s[it + 1].to_lowercase().next().unwrap()
                    && ((s[it].is_uppercase() && s[it + 1].is_lowercase())
                        || (s[it].is_lowercase() && s[it + 1].is_uppercase()))
                {
                    s[it] = '-';
                    s[it + 1] = '-';
                    it += 2;
                    is_string_bad = true;
                } else {
                    it += 1;
                }
            }

            s.retain(|&c| c != '-');
            if !is_string_bad || s.is_empty() {
                break;
            }
        }

        String::from_iter(s)
    }

    fn make_good_smarter(s: String) -> String {
        s.chars()
            .fold(String::new(), |mut result, c2| match (result.pop(), c2) {
                (None, c2) => c2.to_string(),
                (Some(c1), c2) if c1 != c2 && c1.to_lowercase().eq(c2.to_lowercase()) => result,
                (Some(c1), c2) => {
                    result.push(c1);
                    result.push(c2);
                    result
                }
            })
    }
}

fn main() {
    println!("Hello, world!");
}
