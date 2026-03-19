// https://leetcode.com/problems/count-submatrices-with-equal-frequency-of-x-and-y/description/?envType=daily-question&envId=2026-03-19
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let mut prefixes = vec![vec![(0, 0); grid[0].len()]; grid.len()];

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                let prev = if row == 0 {
                    (0, 0)
                } else {
                    prefixes[row - 1][col]
                };
                let delta = match grid[row][col] {
                    'X' => (1, 0),
                    'Y' => (0, 1),
                    _ => (0, 0),
                };
                prefixes[row][col] = (prev.0 + delta.0, prev.1 + delta.1);
            }
        }

        let mut answer = 0;
        for row in 0..prefixes.len() {
            let mut current = (0, 0);
            for (delta_x, delta_y) in prefixes[row].iter().take(prefixes[0].len()) {
                current = (current.0 + delta_x, current.1 + delta_y);
                if current.0 > 0 && current.0 == current.1 {
                    answer += 1;
                }
            }
        }
        answer
    }
}

fn main() {
    let test_cases = [(vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']], 3)];
    for (grid, exp) in test_cases {
        println!("{} exp: {exp}", Solution::number_of_submatrices(grid));
    }
}
