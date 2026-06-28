// https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging/description/
struct Solution {}
#[allow(dead_code)]
impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        arr[0] = 1;

        for i in 1..arr.len() {
            arr[i] = (arr[i - 1] + 1).min(arr[i]);
        }

        *arr.last().unwrap()
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
