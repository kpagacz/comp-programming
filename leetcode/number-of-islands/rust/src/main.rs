// https://leetcode.com/problems/number-of-islands/description/
pub struct Solution;

const NEIGHBOURS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
const VISITED: char = '2';

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut islands = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    islands += 1;
                    Solution::floodfill(&mut grid, i, j);
                }
            }
        }
        islands
    }

    fn floodfill(grid: &mut [Vec<char>], i: usize, j: usize) {
        if grid[i][j] != '1' {
            return;
        }
        grid[i][j] = VISITED;
        let nrows = grid.len() as i32;
        let ncols = grid[0].len() as i32;

        NEIGHBOURS
            .iter()
            .filter_map(|(delta_i, delta_j)| {
                let new_i = delta_i + i as i32;
                let new_j = delta_j + j as i32;
                if new_i >= 0 && new_i < nrows && new_j >= 0 && new_j < ncols {
                    Some((new_i as usize, new_j as usize))
                } else {
                    None
                }
            })
            .for_each(|(i, j)| Solution::floodfill(grid, i, j));
    }
}

fn main() {
    println!("Hello, world!");
}
