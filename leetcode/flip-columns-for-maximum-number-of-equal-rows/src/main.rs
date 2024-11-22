// https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows/description/
pub struct Solution;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut patterns = HashMap::<Vec<usize>, i32>::default();
        for row in matrix {
            *patterns
                .entry(Solution::row_to_bits(&row, row[0] == 1))
                .or_insert(0) += 1;
        }
        *patterns.values().max().unwrap_or(&0) as _
    }

    fn row_to_bits(row: &[i32], inverted: bool) -> Vec<usize> {
        let mut answer = Vec::with_capacity((row.len() + 63) / 64);
        let mut offset = 0;
        let mut current = 0usize;

        for &bit in row {
            if (bit == 1 && !inverted) || (bit == 0 && inverted) {
                current |= 1 << offset;
            }
            offset += 1;

            if offset == 64 {
                offset = 0;
                answer.push(current);
                current = 0;
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
