// https://leetcode.com/problems/count-sub-islands/description/
pub struct Solution;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        assert!(grid1.len() == grid2.len());
        assert!(grid1[0].len() == grid2[0].len());

        let mut stack = vec![];
        let mut count = 0;
        let mut visited = vec![vec![false; grid1[0].len()]; grid1.len()];
        for row in 0..grid1.len() {
            for col in 0..grid1[0].len() {
                const LAND: i32 = 1;
                if grid1[row][col] == LAND && grid2[row][col] == LAND && !visited[row][col] {
                    stack.push((row, col));
                    let mut is_subisland = true;
                    while let Some((row, col)) = stack.pop() {
                        if visited[row][col] {
                            continue;
                        }
                        visited[row][col] = true;
                        match (grid1[row][col] == LAND, grid2[row][col] == LAND) {
                            (true, true) => {
                                if row >= 1 {
                                    stack.push((row - 1, col));
                                }
                                if row < grid1.len() - 1 && grid2[row + 1][col] == LAND {
                                    stack.push((row + 1, col));
                                }
                                if col >= 1 && grid2[row][col - 1] == LAND {
                                    stack.push((row, col - 1));
                                }
                                if col < grid1[0].len() - 1 && grid2[row][col + 1] == LAND {
                                    stack.push((row, col + 1));
                                }
                            }
                            (false, true) => {
                                is_subisland = false;
                                if row >= 1 {
                                    stack.push((row - 1, col));
                                }
                                if row < grid1.len() - 1 && grid2[row + 1][col] == LAND {
                                    stack.push((row + 1, col));
                                }
                                if col >= 1 && grid2[row][col - 1] == LAND {
                                    stack.push((row, col - 1));
                                }
                                if col < grid1[0].len() - 1 && grid2[row][col + 1] == LAND {
                                    stack.push((row, col + 1));
                                }
                            }
                            (true, false) | (false, false) => {}
                        }
                    }
                    if is_subisland {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let test_cases = [
        (
            vec![
                vec![1, 1, 1, 0, 0],
                vec![0, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 1, 1],
            ],
            vec![
                vec![1, 1, 1, 0, 0],
                vec![0, 0, 1, 1, 1],
                vec![0, 1, 0, 0, 0],
                vec![1, 0, 1, 1, 0],
                vec![0, 1, 0, 1, 0],
            ],
            3,
        ),
        (
            vec![
                vec![1, 0, 1, 0, 1],
                vec![1, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 1, 0, 1],
            ],
            vec![
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
                vec![0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0],
                vec![1, 0, 0, 0, 1],
            ],
            2,
        ),
        (
            vec![
                vec![0, 1, 1, 1, 1, 1, 1, 0, 1, 1],
                vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 1],
                vec![1, 0, 1, 1, 0, 1, 1, 1, 1, 1],
                vec![1, 0, 1, 1, 0, 1, 1, 1, 1, 1],
                vec![1, 0, 1, 1, 1, 1, 1, 0, 1, 1],
                vec![1, 1, 0, 0, 1, 1, 1, 0, 0, 1],
                vec![1, 1, 0, 1, 1, 0, 0, 1, 1, 0],
                vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 0, 0, 1, 1, 0, 1, 1, 1, 1],
            ],
            vec![
                vec![0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 0, 0, 1, 1, 1, 0, 0, 1, 0],
                vec![1, 1, 1, 0, 1, 1, 0, 1, 1, 1],
                vec![1, 0, 0, 1, 0, 0, 1, 0, 1, 1],
                vec![0, 1, 1, 1, 0, 1, 0, 1, 1, 0],
                vec![1, 1, 1, 0, 0, 0, 1, 0, 1, 0],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 0, 0, 0, 1, 0, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 0, 1, 1, 0],
            ],
            2,
        ),
    ];

    for (grid1, grid2, expected) in test_cases {
        println!(
            "Got: {}, expected: {expected}",
            Solution::count_sub_islands(grid1, grid2)
        );
    }
}
