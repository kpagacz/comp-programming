// https://leetcode.com/problems/count-the-hidden-sequences/description/?envType=daily-question&envId=2025-04-21
pub struct Solution;

impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut min = 0i64;
        let mut max = 0i64;
        let mut curr = 0i64;
        for diff in differences {
            curr += diff as i64;
            min = min.min(curr);
            max = max.max(curr);
        }
        let diff_span = max - min;
        let allowed_span = (upper - lower) as i64;
        (allowed_span - diff_span + 1).max(0) as _
    }
}

fn main() {
    let test_cases = [(vec![1, -3, 4], 1, 6), (vec![-40], -46, 53)];
    for (differences, lower, upper) in test_cases {
        println!("{}", Solution::number_of_arrays(differences, lower, upper));
    }
}
