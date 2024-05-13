// https://leetcode.com/problems/score-after-flipping-matrix/description/?envType=daily-question&envId=2024-05-13
pub struct Solution;

impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let bits = grid[0].len();
        let n = grid.len();

        for row in grid.iter_mut() {
            if row[0] == 0 {
                row.iter_mut().for_each(|bit| *bit = 1 - *bit);
            }
        }

        let mut power = 1;
        (0..bits)
            .rev()
            .map(|bit| {
                let ones = grid.iter().filter(|row| row[bit] == 1).count();
                let ans = (ones * power).max((n - ones) * power);
                power <<= 1;
                ans
            })
            .sum::<usize>() as _
    }
}

fn main() {
    let test_cases = [vec![vec![0, 1], vec![1, 1]]];
    for grid in test_cases {
        println!("{}", Solution::matrix_score(grid));
    }
}
