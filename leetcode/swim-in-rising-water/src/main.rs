// https://leetcode.com/problems/swim-in-rising-water/description/?envType=daily-question&envId=2025-10-06
struct Solution;

// Interesting problem. Funnily enough I did think about other two solutions to it:
// * union find based
// * a customized djikstra
//
// Funny problem.
#[allow(dead_code)]
impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let (mut left, mut right) = (0, 250 * 250);
        let mut start_covered = vec![vec![false; grid[0].len()]; grid.len()];
        let mut end_covered = vec![vec![false; grid[0].len()]; grid.len()];
        while left < right {
            let middle = left + (right - left) / 2;

            if grid[0][0] <= middle {
                Solution::flood_fill(0, 0, &grid, middle, &mut start_covered);
            }
            if grid[grid.len() - 1][grid[0].len() - 1] <= middle {
                Solution::flood_fill(
                    grid.len() - 1,
                    grid[0].len() - 1,
                    &grid,
                    middle,
                    &mut end_covered,
                );
            }

            let mut is_common = false;
            for row in 0..grid.len() {
                for col in 0..grid[0].len() {
                    if start_covered[row][col] && end_covered[row][col] {
                        is_common = true;
                        break;
                    }
                }
            }

            if is_common {
                right = middle;
            } else {
                left = middle + 1;
            }

            // Reset the visited arrays
            start_covered.fill(vec![false; grid[0].len()]);
            end_covered.fill(vec![false; grid[0].len()]);
        }
        left
    }

    fn flood_fill(
        row: usize,
        col: usize,
        grid: &[Vec<i32>],
        max_height: i32,
        visited: &mut [Vec<bool>],
    ) {
        const NEIGHBOURS: [(usize, usize); 4] = [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)];
        if visited[row][col] {
            return;
        }
        visited[row][col] = true;

        NEIGHBOURS
            .into_iter()
            .filter_map(|(delta_x, delta_y)| {
                let new_x = row.wrapping_add(delta_x);
                let new_y = col.wrapping_add(delta_y);
                if new_x < grid.len() && new_y < grid[0].len() && grid[new_x][new_y] <= max_height {
                    Some((new_x, new_y))
                } else {
                    None
                }
            })
            .for_each(|(row, col)| {
                Solution::flood_fill(row, col, grid, max_height, visited);
            });
    }
}

fn main() {
    let test_cases = [vec![
        vec![0, 1, 2, 3, 4],
        vec![24, 23, 22, 21, 5],
        vec![12, 13, 14, 15, 16],
        vec![11, 17, 18, 19, 20],
        vec![10, 9, 8, 7, 6],
    ]];
    for grid in test_cases {
        println!("{}", Solution::swim_in_water(grid));
    }
}
