// https://leetcode.com/problems/count-number-of-homogenous-substrings/description/
pub struct Solution {}

const MOD: i32 = 10i32.pow(9) + 7;
const usize_MOD: usize = 10usize.pow(9) + 7;
impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let length = s.len();
        let s = s.as_bytes();
        let (mut first, mut last) = (0, 0);

        let mut answer = 0;
        while last < length {
            if s[first] == s[last] {
                if last + 1 < length {
                    last += 1;
                } else {
                    answer = (answer + Solution::substrings_count(last - first + 1)) % MOD;
                    last += 1;
                }
            } else {
                answer = (answer + Solution::substrings_count(last - first)) % MOD;
                first = last;
            }
        }

        answer
    }

    fn substrings_count(length: usize) -> i32 {
        (((length + 1) * length / 2) % usize_MOD) as i32
    }
}
fn main() {
    println!("Hello, world!");
}
