// https://leetcode.com/contest/weekly-contest-372/problems/make-three-strings-equal/
pub struct Solution {}

impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let mut common_prefix = 0;
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        while common_prefix < s1.len()
            && common_prefix < s2.len()
            && common_prefix < s3.len()
            && s1[common_prefix] == s2[common_prefix]
            && s2[common_prefix] == s3[common_prefix]
        {
            common_prefix += 1;
        }
        if common_prefix == 0 {
            -1
        } else {
            let ans = s1.len() + s2.len() + s3.len() - 3 * common_prefix;
            ans as i32
        }
    }
}
fn main() {
    println!("Hello, world!");
}
