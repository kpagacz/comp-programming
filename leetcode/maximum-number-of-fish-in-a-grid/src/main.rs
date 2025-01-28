// https://leetcode.com/problems/maximum-number-of-fish-in-a-grid/description/
pub struct Solution;

impl Solution {
    pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
        const NEIGHBOURS: [(usize, usize); 4] = [(0, usize::MAX), (0, 1), (usize::MAX, 0), (1, 0)];
        fn flood_fill(grid: &mut [Vec<i32>], current: (usize, usize)) -> i32 {
            let (x, y) = current;
            if grid[x][y] == 0 {
                0
            } else {
                let fishes = grid[x][y];
                grid[x][y] = 0;
                let (rows, cols) = (grid.len(), grid[0].len());
                fishes
                    + NEIGHBOURS
                        .into_iter()
                        .filter_map(|(dx, dy)| {
                            let new_x = x + dx;
                            let new_y = y + dy;
                            if new_x < rows && new_y < cols {
                                Some((new_x, new_y))
                            } else {
                                None
                            }
                        })
                        .map(|(x, y)| flood_fill(grid, (x, y)))
                        .sum::<i32>()
            }
        }
        (0..grid.len())
            .map(|row| {
                (0..grid[0].len())
                    .map(|col| flood_fill(&mut grid, (row, col)))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
