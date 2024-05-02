// https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/description/
pub struct Solution;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut positives = vec![false; 1001];
        let mut answer = 1;

        for &num in &nums {
            if num > 0 {
                positives[num as usize] = true;
            }
        }

        for num in nums {
            if num < 0 && positives[-num as usize] {
                answer = answer.min(num);
            }
        }
        -answer
    }
}

fn main() {
    let test_cases = [
        vec![-1, 2, -3, 3],
        vec![-1, 10, 6, 7, -7, 1],
        vec![-10, 8, 6, 7, -2, -3],
    ];
    for nums in test_cases {
        println!("{}", Solution::find_max_k(nums));
    }
}
