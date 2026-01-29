// https://leetcode.com/problems/minimum-cost-path-with-teleportations/description/?envType=daily-question&envId=2026-01-28
struct Solution;
#[allow(dead_code)]
impl Solution {
    // This does not actually work - it gets TLE
    // It needs an improvement - do not store the state of left
    // teleportations in the PQ, but at the end of the djikstra
    // run, create a new PQ. How?
    // The cost c of moving to point (i,j) given one teleportation
    // is min_(grid[x][y] >= grid[i][j]) costs[x][y]
    // which is to say it's the minimum cost of moving to any cell
    // (x,y) that has >= value than cell (i,j).
    // If we sort the cells based on their values and walk through them
    // maintaining the minimum cost outside of the loop we can find the
    // min cost values and put them in the new pq.
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut sorted_grid_points =
            grid.iter()
                .enumerate()
                .fold(Vec::new(), |mut arr, (row_id, row)| {
                    arr.extend(
                        row.iter()
                            .enumerate()
                            .map(|(col_id, val)| (*val, row_id, col_id)),
                    );
                    arr
                });
        sorted_grid_points.sort_unstable_by_key(|grid_point| grid_point.0);

        type State = (Reverse<i32>, usize, usize, i32); // (cost, x, y, teleportations left)
        let mut pq: BinaryHeap<State> = BinaryHeap::new();
        pq.push((Reverse(0), 0, 0, k));
        let mut visited = vec![vec![vec![false; k as usize + 1]; grid[0].len()]; grid.len()];

        while let Some((Reverse(cost), x, y, teleportations_left)) = pq.pop() {
            if x == grid.len() - 1 && y == grid[0].len() - 1 {
                return cost;
            }
            if visited[x][y][teleportations_left as usize] {
                continue;
            } else {
                visited[x][y][teleportations_left as usize] = true;
            }

            // Move right
            if y + 1 < grid[0].len() && !visited[x][y + 1][teleportations_left as usize] {
                pq.push((
                    Reverse(cost + grid[x][y + 1]),
                    x,
                    y + 1,
                    teleportations_left,
                ));
            }
            // Move down
            if x + 1 < grid.len() && !visited[x + 1][y][teleportations_left as usize] {
                pq.push((
                    Reverse(cost + grid[x + 1][y]),
                    x + 1,
                    y,
                    teleportations_left,
                ));
            }
            // Teleport
            if teleportations_left > 0 {
                let teleportation_targets_past_end =
                    sorted_grid_points.partition_point(|(cost, _, _)| *cost <= grid[x][y]);
                sorted_grid_points[..teleportation_targets_past_end]
                    .iter()
                    .filter(|&(_, new_x, new_y)| {
                        !visited[*new_x][*new_y][teleportations_left as usize - 1]
                    })
                    .for_each(|(_, new_x, new_y)| {
                        pq.push((Reverse(cost), *new_x, *new_y, teleportations_left - 1));
                    });
            }
        }

        unreachable!("m-1 n-1 is always reachable")
    }
}

fn main() {
    let test_cases = [
        (vec![vec![1, 3, 3], vec![2, 5, 4], vec![4, 3, 5]], 2, 7),
        (
            vec![
                vec![7, 10, 5, 13, 14, 2, 5],
                vec![14, 10, 18, 12, 19, 16, 4],
                vec![9, 24, 10, 21, 18, 12, 10],
                vec![13, 15, 25, 32, 26, 25, 27],
                vec![29, 17, 35, 36, 20, 29, 35],
            ],
            1,
            72,
        ),
        (
            vec![
                vec![9, 2, 7, 9, 17, 11, 6, 23, 24],
                vec![17, 10, 19, 8, 19, 18, 27, 16, 23],
            ],
            2,
            23,
        ),
        (
            vec![
                vec![19, 10],
                vec![23, 13],
                vec![16, 32],
                vec![38, 41],
                vec![30, 36],
                vec![53, 31],
            ],
            1,
            55,
        ),
    ];
    for (grid, k, exp) in test_cases {
        println!("{} exp {exp}", Solution::min_cost(grid, k));
    }
}
