// https://leetcode.com/problems/zero-array-transformation-i/description/
pub struct Solution;

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut deltas = vec![0; nums.len() + 1];

        for query in queries {
            let (left, right) = (query[0] as usize, query[1] as usize);
            deltas[left] -= 1;
            deltas[right + 1] += 1;
        }

        let mut current_delta = 0;
        for i in 0..nums.len() {
            current_delta += deltas[i];
            if nums[i] + current_delta > 0 {
                return false;
            }
        }

        true
    }
}

fn main() {
    println!("Hello, world!");
}
