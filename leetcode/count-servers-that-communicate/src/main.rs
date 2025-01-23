// https://leetcode.com/problems/count-servers-that-communicate/description/
pub struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        let mut colmap = HashMap::new();
        let mut rowmap = HashMap::new();

        (0..grid.len()).for_each(|row| {
            (0..grid[0].len()).for_each(|col| {
                if grid[row][col] == 1 {
                    *colmap.entry(col).or_insert(0) += 1;
                    *rowmap.entry(row).or_insert(0) += 1;
                }
            })
        });

        let mut answer = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1
                    && (colmap.get(&j).is_some_and(|count| *count > 1)
                        || rowmap.get(&i).is_some_and(|count| *count > 1))
                {
                    answer += 1;
                }
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
