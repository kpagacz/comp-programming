// https://leetcode.com/problems/reverse-string-ii/description/
pub struct Solution;

impl Solution {
    pub fn reverse_str(mut s: String, k: i32) -> String {
        let s = unsafe { s.as_bytes_mut() };
        let k = k as usize;

        let mut it = k;

        loop {
            if it > s.len() {
                let (mut start, mut end) = (it - k, s.len() - 1);
                while start < end {
                    s.swap(start, end);
                    start += 1;
                    end -= 1;
                }
                break;
            } else {
                let (mut start, mut end) = (it - k, it - 1);
                while start < end {
                    s.swap(start, end);
                    start += 1;
                    end -= 1;
                }
            }
            it += 2 * k;
        }

        String::from_utf8(s.to_vec()).unwrap()
    }

    pub fn reverse_str_smarter(s: String, k: i32) -> String {
        let k = k as usize;
        let mut s: Vec<_> = s.chars().collect();

        (0..).take(s.len()).step_by(2 * k).for_each(|i| {
            if let Some(slice) = s.get_mut(i..i + k) {
                slice.reverse();
            } else {
                s[i..].reverse();
            }
        });

        s.iter().collect()
    }
}

fn main() {
    let test_cases = [("abcdefg", 2), ("1234567890abcd", 3)];
    for (s, k) in test_cases {
        println!("{}", Solution::reverse_str(s.to_owned(), k));
    }
}
