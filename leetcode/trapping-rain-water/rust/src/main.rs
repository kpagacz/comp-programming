// https://leetcode.com/problems/trapping-rain-water/description/
pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut monotonic_stack = vec![];
        let mut answer = 0;

        for (pos, &col) in height.iter().enumerate() {
            while !monotonic_stack.is_empty() && col > height[*monotonic_stack.last().unwrap()] {
                let popped = monotonic_stack.pop().unwrap();
                if let Some(left_border) = monotonic_stack.last() {
                    let width = pos - left_border - 1;
                    let height = col.min(height[*left_border]) - height[popped];
                    answer += width as i32 * height;
                }
            }

            monotonic_stack.push(pos);
        }

        answer
    }
}

fn main() {
    let test_cases = [
        vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
        vec![4, 2, 0, 3, 2, 5],
        vec![1, 2, 3],
    ];

    for height in test_cases {
        println!("{}", Solution::trap(height));
    }
}
