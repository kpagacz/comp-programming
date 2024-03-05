// https://leetcode.com/problems/minimum-length-of-string-after-deleting-similar-ends/description/
pub struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        if s.len() == 1 {
            return 1;
        }
        let mut left = 0;
        let mut right = s.len() - 1;
        let s: Vec<_> = s.chars().collect();

        if s[left] != s[right] {
            return s.len() as i32;
        } else {
            let common = &s[left];

            while left + 1 < right && s[left + 1] == *common {
                left += 1;
            }

            while right - 1 > left && s[right - 1] == *common {
                right -= 1;
            }
        }

        // left and right are indices of the last removed characters
        while left + 1 < right {
            // println!(
            //     "left: {left} char: {}, right: {right} char: {}",
            //     s[left], s[right]
            // );
            if s[left + 1] == s[right - 1] && left + 1 != right - 1 {
                left += 1;
                if left < right - 1 {
                    right -= 1;
                }
                let common = &s[left];

                while left + 1 < right && s[left + 1] == *common {
                    left += 1;
                }

                while right - 1 > left && s[right - 1] == *common {
                    right -= 1;
                }
            } else {
                break;
            }
        }

        right as i32 - left as i32 - 1
    }
}

fn main() {
    let tests = vec![
        "ca",
        "cabaabac",
        "aabccabba",
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbabbbbbbbbbbbbbbbccbcbcbccbbabbb",
    ];
    for test in tests {
        println!("Word: {test}");
        println!("{}", Solution::minimum_length(test.to_owned()));
    }
}
