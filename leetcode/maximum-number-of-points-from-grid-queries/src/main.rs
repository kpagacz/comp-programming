// https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/description/?envType=daily-question&envId=2025-03-28
pub struct Solution;

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        fn smart_rec(
            grid: &[Vec<i32>],
            query_value: i32,
            x: usize,
            y: usize,
            visited: &mut [Vec<bool>],
            ending_points: &mut Vec<(usize, usize)>,
            sum: &mut i32,
        ) {
            visited[x][y] = true;
            if query_value > grid[x][y] {
                *sum += 1;
                // println!("Increasing sum and going forward for {x} {y}");
                for (delta_x, delta_y) in [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)] {
                    let new_x = x.wrapping_add(delta_x);
                    let new_y = y.wrapping_add(delta_y);

                    if new_x < grid.len() && new_y < grid[0].len() && !visited[new_x][new_y] {
                        smart_rec(grid, query_value, new_x, new_y, visited, ending_points, sum);
                    }
                }
            } else {
                // println!("That's it {x} {y}");
                ending_points.push((x, y));
            }
        }

        let mut answer = vec![0; queries.len()];
        let mut queries_sorted: Vec<_> = (0..queries.len()).collect();
        queries_sorted.sort_by_key(|&pos| queries[pos]);
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let grid_max = *grid
            .iter()
            .map(|row| row.iter().max().unwrap())
            .max()
            .unwrap();

        let mut sum = 0;
        let mut ending_points = vec![(0usize, 0usize)];
        for i in 0..queries_sorted.len() {
            let pos_in_queries = queries_sorted[i];
            let query_value = queries[pos_in_queries];
            if query_value > grid_max {
                answer[pos_in_queries] = grid.len() as i32 * grid[0].len() as i32;
                continue;
            }
            if i > 0 && query_value == queries[queries_sorted[i - 1]] {
                answer[pos_in_queries] = answer[queries_sorted[i - 1]];
                continue;
            }

            let mut new_ending_points = Vec::default();
            for (x, y) in ending_points {
                smart_rec(
                    &grid,
                    query_value,
                    x,
                    y,
                    &mut visited,
                    &mut new_ending_points,
                    &mut sum,
                );
            }
            ending_points = new_ending_points;
            // println!("query: {query_value} Sum: {sum}. new ending points: {ending_points:?}");
            answer[pos_in_queries] = sum;
        }

        answer
    }
}

fn main() {
    let test_cases = [(
        vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
        vec![5, 6, 2],
    )];
    for (grid, queries) in test_cases {
        println!("{:?}", Solution::max_points(grid, queries));
    }
}
