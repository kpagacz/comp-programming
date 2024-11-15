// https://leetcode.com/problems/shortest-subarray-to-be-removed-to-make-array-sorted/description/
pub struct Solution;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut non_dec_prefix_end = 0;
        while non_dec_prefix_end < n - 1 && arr[non_dec_prefix_end] <= arr[non_dec_prefix_end + 1] {
            non_dec_prefix_end += 1;
        }
        if non_dec_prefix_end == n - 1 {
            return 0;
        }

        let mut non_dec_suffix_start = n - 1;
        while non_dec_suffix_start > 0 && arr[non_dec_suffix_start] >= arr[non_dec_suffix_start - 1]
        {
            non_dec_suffix_start -= 1;
        }

        let mut answer = non_dec_suffix_start;
        let mut end = non_dec_suffix_start;

        for start in 0..=non_dec_prefix_end {
            while end < n && arr[start] > arr[end] {
                end += 1;
            }
            answer = answer.min(end - start - 1);
        }
        answer as _
    }
}

fn main() {
    println!("Hello, world!");
}
