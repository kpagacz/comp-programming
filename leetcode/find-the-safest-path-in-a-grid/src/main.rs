// https://leetcode.com/problems/find-the-safest-path-in-a-grid/description/
pub struct Solution;

impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;

        let n = grid.len();
        let is_valid = |row: usize, col: usize| row < n && col < n;
        let mut queue = VecDeque::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    queue.push_back((i, j, 1));
                    grid[i][j] = 0;
                } else {
                    grid[i][j] = -1;
                }
            }
        }

        while let Some((row, col, safety)) = queue.pop_front() {
            let neighbours = [
                (row + 1, col),
                (row - 1, col),
                (row, col + 1),
                (row, col - 1),
            ];
            for (i, j) in neighbours {
                if is_valid(i, j) && grid[i][j] == -1 {
                    grid[i][j] = safety;
                    queue.push_back((i, j, safety + 1));
                }
            }
        }

        let mut min_safety = grid[0][0];
        queue.push_back((0, 0, grid[0][0]));
        while let Some((row, col, safety)) = queue.pop_front() {
            min_safety = i32::min(min_safety, safety);
            if row == n - 1 && col == n - 1 {
                break;
            }
            let neighbours = [
                (row + 1, col),
                (row - 1, col),
                (row, col + 1),
                (row, col - 1),
            ];

            for (r, c) in neighbours {
                if is_valid(r, c) && grid[r][c] != -1 {
                    let safety = grid[r][c];
                    grid[r][c] = -1;
                    if safety < min_safety {
                        queue.push_back((r, c, safety));
                    } else {
                        queue.push_front((r, c, safety));
                    }
                }
            }
        }
        min_safety
    }
}

fn main() {
    let test_cases = [vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]]];
    for grid in test_cases {
        println!("{}", Solution::maximum_safeness_factor(grid));
    }
}
