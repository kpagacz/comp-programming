// https://leetcode.com/problems/trapping-rain-water-ii/description/
pub struct Solution;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let (rows, cols) = (height_map.len(), height_map[0].len());
        let mut visited = vec![vec![false; cols]; rows];
        const NEIGHBOURS: [(usize, usize); 4] = [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)];

        let mut border = BinaryHeap::new();
        (0..rows).for_each(|row| {
            border.push(Reverse((height_map[row][0], row, 0)));
            visited[row][0] = true;
            border.push(Reverse((height_map[row][cols - 1], row, cols - 1)));
            visited[row][cols - 1] = true;
        });
        (0..cols).for_each(|col| {
            border.push(Reverse((height_map[0][col], 0, col)));
            visited[0][col] = true;
            border.push(Reverse((height_map[rows - 1][col], rows - 1, col)));
            visited[rows - 1][col] = true;
        });

        let mut trapped_water = 0;
        while let Some(Reverse((min_height, row, col))) = border.pop() {
            for (new_row, new_col) in NEIGHBOURS
                .into_iter()
                .map(|(dx, dy)| (row.wrapping_add(dx), col.wrapping_add(dy)))
            {
                if new_row < rows && new_col < cols && !visited[new_row][new_col] {
                    trapped_water += (min_height - height_map[new_row][new_col]).max(0);
                    visited[new_row][new_col] = true;
                    border.push(Reverse((
                        height_map[new_row][new_col].max(min_height),
                        new_row,
                        new_col,
                    )));
                }
            }
        }
        trapped_water
    }
}

fn main() {
    println!("Hello, world!");
}
