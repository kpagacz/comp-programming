// https://leetcode.com/problems/minimum-swaps-to-arrange-a-binary-grid/?envType=daily-question&envId=2026-03-02
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let mut right_one = vec![];
        for row in &grid {
            let rightmost_one = row.iter().rposition(|n| *n == 1);
            if let Some(rightmost_one) = rightmost_one {
                right_one.push(rightmost_one);
            } else {
                right_one.push(0);
            }
        }
        let mut answer = 0;
        for i in 0..grid.len() {
            let first_suitable = right_one[i..].iter().position(|rightmost| *rightmost <= i);
            if let Some(first_suitable) = first_suitable {
                answer += first_suitable;
                for swapped in (i..i + first_suitable).rev() {
                    right_one.swap(swapped, swapped + 1);
                }
            } else {
                return -1;
            }
        }
        answer as _
    }
}

fn main() {
    let test_cases = [
        (vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]], 3),
        (vec![vec![0, 0], vec![0, 1]], 0),
    ];

    for (grid, exp) in test_cases {
        println!("{} exp: {exp}", Solution::min_swaps(grid));
    }
}
