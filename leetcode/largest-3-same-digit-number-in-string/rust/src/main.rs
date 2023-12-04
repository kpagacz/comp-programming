// https://leetcode.com/problems/largest-3-same-digit-number-in-string/
pub struct Solution {}

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        num.as_bytes()
            .windows(3)
            .fold(String::from(""), |acc, window| {
                if window[0] == window[1] && window[1] == window[2] {
                    if acc.is_empty() {
                        String::from_utf8(window.to_vec()).unwrap()
                    } else {
                        if acc.parse::<usize>().unwrap()
                            < String::from_utf8(window.to_vec())
                                .unwrap()
                                .parse::<usize>()
                                .unwrap()
                        {
                            String::from_utf8(window.to_vec()).unwrap()
                        } else {
                            acc
                        }
                    }
                } else {
                    acc
                }
            })
    }
}
fn main() {
    println!("Hello, world!");
}
