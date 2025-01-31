// https://leetcode.com/problems/making-a-large-island/description/
pub struct Solution;

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let colcount = grid[0].len();

        fn find(parent: &mut [usize], node: usize) -> usize {
            if parent[node] == usize::MAX {
                node
            } else {
                let found = find(parent, parent[node]);
                parent[node] = found;
                found
            }
        }
        fn union(parent: &mut [usize], first: usize, second: usize) {
            let first_root = find(parent, first);
            let second_root = find(parent, second);

            if first_root != second_root {
                parent[second_root] = first_root;
            }
        }
        let mut parent = vec![usize::MAX; colcount * grid.len()];
        const NEIGHBOURS: [(usize, usize); 4] = [(0, usize::MAX), (0, 1), (usize::MAX, 0), (1, 0)];
        for row in 0..grid.len() {
            for col in 0..colcount {
                if grid[row][col] == 0 {
                    continue;
                }
                for (n_x, n_y) in NEIGHBOURS.into_iter().filter_map(|(dx, dy)| {
                    let (new_x, new_y) = (row.wrapping_add(dx), col.wrapping_add(dy));
                    if new_x < grid.len() && new_y < colcount && grid[new_x][new_y] == 1 {
                        Some((new_x, new_y))
                    } else {
                        None
                    }
                }) {
                    union(&mut parent, row * colcount + col, n_x * colcount + n_y);
                }
            }
        }
        let mut areas = vec![0; parent.len()];
        let mut max = 0;
        for id in 0..parent.len() {
            let (x, y) = (id / grid.len(), id % colcount);
            if grid[x][y] == 1 {
                let par = find(&mut parent, id);
                areas[par] += 1;
                max = max.max(areas[par]);
            }
        }

        for row in 0..grid.len() {
            for col in 0..colcount {
                if grid[row][col] == 1 {
                    continue;
                }

                let mut neighbouring_island = NEIGHBOURS
                    .into_iter()
                    .filter_map(|(dx, dy)| {
                        let (nx, ny) = (row.wrapping_add(dx), col.wrapping_add(dy));
                        if nx < grid.len() && ny < colcount && grid[nx][ny] == 1 {
                            Some((nx, ny))
                        } else {
                            None
                        }
                    })
                    .map(|(nx, ny)| find(&mut parent, nx * colcount + ny))
                    .collect::<Vec<_>>();

                neighbouring_island.sort();
                neighbouring_island.dedup();
                max = max.max(
                    neighbouring_island
                        .into_iter()
                        .map(|root| areas[root])
                        .sum::<i32>()
                        + 1,
                );
            }
        }
        max
    }
}

fn main() {
    let test_cases = [vec![vec![1, 0], vec![0, 1]]];
    for grid in test_cases {
        println!("{}", Solution::largest_island(grid));
    }
}
