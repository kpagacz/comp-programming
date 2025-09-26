// https://leetcode.com/problems/valid-triangle-number/description/?envType=daily-question&envId=2025-09-26
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.retain(|length| *length > 0);
        if nums.len() < 3 {
            return 0;
        }
        nums.sort_unstable();
        let mut answer = 0;

        for first in 0..nums.len() {
            for second in first + 1..nums.len() {
                // Find the first bigger equal
                let sum = nums[first] + nums[second];
                let first_bigger_equal = nums.partition_point(|length| *length < sum);
                let possible = first_bigger_equal - second - 1;
                answer += possible;
            }
        }

        answer as _
    }
}

fn main() {
    println!("Hello, world!");
}
