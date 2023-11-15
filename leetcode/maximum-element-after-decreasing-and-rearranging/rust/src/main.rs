// https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging/description/
pub struct Solution {}

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        let mut min = 0;
        for num in arr {
            min = (min + 1).min(num);
        }
        min
    }
}
fn main() {
    let test_cases = vec![vec![2, 2, 1, 2, 1], vec![100, 1, 1000], vec![1, 2, 3, 4, 5]];
    for arr in test_cases {
        println!(
            "{}",
            Solution::maximum_element_after_decrementing_and_rearranging(arr)
        );
    }
}
