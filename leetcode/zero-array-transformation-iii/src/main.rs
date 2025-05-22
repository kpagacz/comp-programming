// https://leetcode.com/problems/zero-array-transformation-iii/description/?envType=daily-question&envId=2025-05-22
pub struct Solution;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        queries.sort();

        use std::collections::BinaryHeap;
        let mut rightmost_queries = BinaryHeap::new();
        let mut query_it = 0usize;
        let mut delta = 0;
        let mut delta_array = vec![0; nums.len() + 1];

        for (i, num) in nums.into_iter().enumerate() {
            delta += delta_array[i];
            while query_it < queries.len() && i >= queries[query_it][0] as usize {
                let curr_query = &queries[query_it];
                rightmost_queries.push(curr_query[1]);
                query_it += 1;
            }

            while delta < num
                && !rightmost_queries.is_empty()
                && *rightmost_queries.peek().unwrap() as usize >= i
            {
                delta += 1;
                let end = rightmost_queries.pop().unwrap() as usize;
                delta_array[end + 1] -= 1;
            }

            if delta < num {
                return -1;
            }
        }

        rightmost_queries.len() as _
    }
}

fn main() {
    println!("Hello, world!");
}
