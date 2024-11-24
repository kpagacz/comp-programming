// https://leetcode.com/problems/maximum-matrix-sum/description/
pub struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut negative = 0usize;
        let mut is_there_zero = false;
        let mut smallest_positive = i32::MAX;
        let mut highest_negative = i32::MIN;
        let mut sum = 0i64;

        for row in matrix {
            for num in row {
                if num < 0 {
                    negative += 1;
                    if num > highest_negative {
                        highest_negative = num
                    }
                } else if num == 0 {
                    is_there_zero = true;
                } else if num < smallest_positive {
                    smallest_positive = num;
                }
                sum += num.abs() as i64;
            }
        }

        if negative % 2 == 0 || is_there_zero {
            sum
        } else {
            if smallest_positive < highest_negative.abs() {
                sum -= 2 * smallest_positive as i64;
            } else {
                sum += 2 * highest_negative as i64;
            }
            sum
        }
    }
}

fn main() {
    println!("Hello, world!");
}
