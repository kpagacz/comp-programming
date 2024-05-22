// https://leetcode.com/problems/special-array-ii/description/
pub struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let ids: Vec<_> = (0..nums.len()).collect();
        let bad_pairs: Vec<_> = ids
            .windows(2)
            .filter_map(|window| {
                if (nums[window[0]] ^ nums[window[1]]) & 1 == 0 {
                    Some(window[0] as i32)
                } else {
                    None
                }
            })
            .collect();

        let mut answer = Vec::with_capacity(queries.len());
        for query in queries {
            let start_in_bad_pairs = bad_pairs.partition_point(|&x| x < query[0]);
            let end_in_bad_pairs = bad_pairs.partition_point(|&x| x < query[1]);

            answer.push(end_in_bad_pairs - start_in_bad_pairs == 0);
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
