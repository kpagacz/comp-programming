// https://leetcode.com/problems/path-with-maximum-gold/description/
pub struct Solution;

const NEIGHBOURS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut [Vec<i32>], mut total_so_far: i32, row: i32, col: i32) -> i32 {
            if row < 0 || row >= grid.len() as i32 || col < 0 || col >= grid[0].len() as i32 {
                return total_so_far;
            }
            let mem;
            if grid[row as usize][col as usize] == 0 {
                return total_so_far;
            } else {
                total_so_far += grid[row as usize][col as usize];
                mem = grid[row as usize][col as usize];
                grid[row as usize][col as usize] = 0;
            }

            total_so_far = NEIGHBOURS
                .iter()
                .map(|(delta_x, delta_y)| dfs(grid, total_so_far, row + delta_x, col + delta_y))
                .max()
                .unwrap();

            grid[row as usize][col as usize] = mem;
            total_so_far
        }

        let mut maximum_gold = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                maximum_gold = maximum_gold.max(dfs(&mut grid, 0, i as i32, j as i32));
            }
        }

        maximum_gold
    }
}

fn main() {
    println!("Hello, world!");
}
