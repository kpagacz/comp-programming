// https://leetcode.com/problems/minimum-number-of-days-to-disconnect-island/description/?envType=daily-question&envId=2024-08-11
pub struct Solution;

impl Solution {
    pub fn min_days(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut marker = 1;
        let mut islands = 0;

        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if grid[x][y] == 1 {
                    marker += 1;
                    Self::flood_fill(&mut grid, x, y, marker, &mut islands);
                }
            }
        }
        // Subtract one to offset the initial one
        marker -= 1;

        // There are already disconnected islands
        // or there are no islands
        if marker != 1 {
            return 0;
        }

        // If there is only one islands, just remove it
        if islands == 1 {
            return 1;
        }

        // Find the articulation point: https://cp-algorithms.com/graph/cutpoints.html
        fn dfs(
            visited: &mut [Vec<bool>],
            discovery_time: &mut [Vec<i32>],
            dp: &mut [Vec<i32>],
            x: usize,
            y: usize,
            parent_x: usize,
            parent_y: usize,
            timer: &mut i32,
            grid: &[Vec<i32>],
        ) -> Option<(usize, usize)> {
            visited[x][y] = true;
            *timer += 1;
            discovery_time[x][y] = *timer;
            dp[x][y] = *timer;

            let mut children = 0;
            let neighbours = Solution::get_neighbours(grid, x, y);
            for (new_x, new_y) in neighbours {
                if parent_x == new_x && parent_y == new_y {
                    continue;
                }
                if visited[new_x][new_y] {
                    dp[x][y] = dp[x][y].min(discovery_time[new_x][new_y]);
                } else {
                    let found = dfs(visited, discovery_time, dp, new_x, new_y, x, y, timer, grid);
                    if found.is_some() {
                        return found;
                    }
                    dp[x][y] = dp[x][y].min(dp[new_x][new_y]);
                    if dp[new_x][new_y] >= discovery_time[x][y] && parent_x != usize::MAX {
                        return Some((x, y));
                    }
                    children += 1;
                }
            }

            if parent_x == usize::MAX && children > 1 {
                Some((x, y))
            } else {
                None
            }
        }

        let mut timer = 0;
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut dp = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
        let mut discovery_time = vec![vec![i32::MAX; grid[0].len()]; grid.len()];

        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if grid[x][y] != 0 && !visited[x][y] {
                    if let Some(_) = dfs(
                        &mut visited,
                        &mut discovery_time,
                        &mut dp,
                        x,
                        y,
                        usize::MAX,
                        usize::MAX,
                        &mut timer,
                        &grid,
                    ) {
                        return 1;
                    }
                }
            }
        }
        2
    }

    fn get_neighbours(grid: &[Vec<i32>], x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if x < grid.len() - 1 {
            neighbours.push((x + 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < grid[0].len() - 1 {
            neighbours.push((x, y + 1));
        }
        neighbours
            .into_iter()
            .filter(|&(x, y)| grid[x][y] != 0)
            .collect()
    }

    fn flood_fill(grid: &mut Vec<Vec<i32>>, x: usize, y: usize, marker: i32, islands: &mut i32) {
        const WATER: i32 = 0;
        if grid[x][y] == WATER || grid[x][y] == marker {
            return;
        }
        grid[x][y] = marker;
        *islands += 1;

        if x > 0 {
            Self::flood_fill(grid, x - 1, y, marker, islands);
        }
        if x < grid.len() - 1 {
            Self::flood_fill(grid, x + 1, y, marker, islands);
        }
        if y > 0 {
            Self::flood_fill(grid, x, y - 1, marker, islands);
        }
        if y < grid[0].len() - 1 {
            Self::flood_fill(grid, x, y + 1, marker, islands);
        }
    }
}

fn main() {
    let test_cases = [
        vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]],
        vec![vec![1, 1]],
        vec![vec![1, 0, 1]],
        vec![vec![1, 0, 1], vec![1, 1, 1]],
        vec![vec![0, 1, 1, 1], vec![0, 1, 1, 1], vec![0, 1, 1, 1]],
        vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
    ];

    for grid in test_cases {
        println!("{}", Solution::min_days(grid));
    }
}
