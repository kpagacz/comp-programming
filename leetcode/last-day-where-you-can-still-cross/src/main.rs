// https://leetcode.com/problems/last-day-where-you-can-still-cross/description/?envType=daily-question&envId=2025-12-31
struct Solution;

const LAND: u8 = 0;
const WATER: u8 = 1;
const DIRECTIONS: [(usize, usize); 4] = [(usize::MAX, 0), (0, usize::MAX), (0, 1), (1, 0)];
#[allow(dead_code)]
impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let mut left = 0;
        let mut right = cells.len();
        let mut grid = vec![vec![LAND; col as usize]; row as usize];

        while left < right {
            let middle = left + (right - left) / 2;

            grid.iter_mut().for_each(|arr| arr.fill(LAND));
            Self::apply_water(&mut grid, &cells[0..middle]);

            if Self::is_crossing_possible(&grid) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left as i32 - 1
    }

    fn is_crossing_possible(grid: &[Vec<u8>]) -> bool {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        for start_col in 0..grid[0].len() {
            if visited[0][start_col] || grid[0][start_col] == WATER {
                continue;
            }
            if Self::dfs(grid, (0, start_col), &mut visited) {
                return true;
            }
        }
        false
    }

    fn dfs(grid: &[Vec<u8>], start: (usize, usize), visited: &mut [Vec<bool>]) -> bool {
        if visited[start.0][start.1] {
            return false;
        }
        if start.0 == grid.len() - 1 {
            return true;
        }
        visited[start.0][start.1] = true;

        DIRECTIONS.into_iter().any(|(dx, dy)| {
            let (new_x, new_y) = (start.0.wrapping_add(dx), start.1.wrapping_add(dy));
            if new_x < grid.len() && new_y < grid[0].len() && grid[new_x][new_y] == LAND {
                Self::dfs(grid, (new_x, new_y), visited)
            } else {
                false
            }
        })
    }

    fn apply_water(grid: &mut [Vec<u8>], cells: &[Vec<i32>]) {
        for cell in cells {
            let (row, col) = (cell[0] as usize - 1, cell[1] as usize - 1);
            grid[row][col] = WATER;
        }
    }
}

fn main() {
    let test_cases = [
        (
            2,
            2,
            vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2]],
            2,
        ),
        (
            2,
            2,
            vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]],
            1,
        ),
    ];
    for (row, col, cells, exp) in test_cases {
        println!(
            "{}, exp: {exp}",
            Solution::latest_day_to_cross(row, col, cells)
        );
    }
}
