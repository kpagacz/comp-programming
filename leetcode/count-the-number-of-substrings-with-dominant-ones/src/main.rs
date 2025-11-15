// https://leetcode.com/problems/count-the-number-of-substrings-with-dominant-ones/description/?envType=daily-question&envId=2025-11-15
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut pre_zero = vec![0; s.len() + 1];
        for i in 2..pre_zero.len() {
            if s[i - 2] == b'0' {
                pre_zero[i] = i - 1;
            } else {
                pre_zero[i] = pre_zero[i - 1];
            }
        }

        let mut count = 0;
        for end in 1..=s.len() {
            let mut zeros = if s[end - 1] == b'0' { 1 } else { 0 };
            let mut left = end;

            while left > 0 && zeros * zeros <= s.len() {
                let ones = end - pre_zero[left] - zeros;
                if ones >= zeros * zeros {
                    // ones >= zeros * zeros
                    // ones - zeros * zeros >= 0
                    // => treat max as a parameter
                    // ones - zeros * zeros - max = -1
                    // max = ones - zeros * zeros + 1
                    count += (left - pre_zero[left]).min(ones - zeros * zeros + 1);
                }
                zeros += 1;
                left = pre_zero[left];
            }
        }
        count as _
    }
}

fn main() {
    let test_cases = [("00011", 5)];
    for (s, exp) in test_cases {
        let s = s.to_owned();
        println!("{}  exp: 5", Solution::number_of_substrings(s.clone()));
    }
}
