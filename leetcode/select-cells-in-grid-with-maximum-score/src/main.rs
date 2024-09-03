// https://leetcode.com/problems/select-cells-in-grid-with-maximum-score/description/
pub struct Solution;

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut mem = vec![vec![-1; 1024]; 101];
        let mut nums_to_rows = vec![vec![]; 101];

        (0..grid.len()).for_each(|row| {
            grid[row]
                .iter()
                .for_each(|&num| nums_to_rows[num as usize].push(row));
        });

        Self::rec(100, 0, &nums_to_rows, &mut mem)
    }

    fn rec(num: usize, mask: usize, nums_to_rows: &[Vec<usize>], mem: &mut [Vec<i32>]) -> i32 {
        if num == 0 {
            return 0;
        }
        if mem[num][mask] != -1 {
            return mem[num][mask];
        }

        let mut max = Self::rec(num - 1, mask, nums_to_rows, mem);

        for &row in &nums_to_rows[num] {
            if (1 << row) & mask == 0 {
                max =
                    max.max(num as i32 + Self::rec(num - 1, mask | (1 << row), nums_to_rows, mem));
            }
        }

        mem[num][mask] = max;
        max
    }
}

fn main() {
    println!("Hello, world!");
}
