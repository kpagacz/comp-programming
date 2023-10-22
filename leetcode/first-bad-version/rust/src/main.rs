// https://leetcode.com/problems/first-bad-version/description/
pub struct Solution {}
// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut left, mut right) = (0, n);
        while left < right {
            let middle = left + (right - left) / 2;
            if self.isBadVersion(middle) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        left
    }
    // fn isBadVersion(&self, version: i32) -> bool {
    //     false
    // }
}
fn main() {
    println!("Hello, world!");
}
