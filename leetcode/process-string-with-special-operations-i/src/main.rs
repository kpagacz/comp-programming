// https://leetcode.com/problems/process-string-with-special-operations-i/description/?envType=daily-question&envId=2026-06-16
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn process_str(s: String) -> String {
        let mut res = vec![];

        for &c in s.as_bytes() {
            match c {
                b'*' => {
                    res.pop();
                }
                b'#' => {
                    res.extend_from_within(..);
                }
                b'%' => {
                    res.reverse();
                }
                _ => {
                    res.push(c);
                }
            }
        }

        String::from_utf8(res).unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
