// https://leetcode.com/problems/bitwise-ors-of-subarrays/description/?envType=daily-question&envId=2025-07-31
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut answer: HashSet<i32> = HashSet::new();
        let mut current_ors = HashSet::new();

        for num in arr {
            let mut new_ors = HashSet::new();
            new_ors.insert(num);
            for previous_or in current_ors {
                new_ors.insert(previous_or | num);
            }
            answer.extend(&new_ors);
            current_ors = new_ors;
        }

        answer.len() as _
    }
}

fn main() {
    println!("Hello, world!");
}
