// https://leetcode.com/problems/length-of-longest-v-shaped-diagonal-segment/description/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        const DIRECTIONS: [(usize, usize); 4] = [
            (1, 1),
            (1, usize::MAX),
            (usize::MAX, usize::MAX),
            (usize::MAX, 1),
        ];
        type Cache = HashMap<(usize, usize, usize, i32, bool), i32>;
        fn rec(
            x: usize,
            y: usize,
            direction: usize,
            target: i32,
            turn_made: bool,
            grid: &[Vec<i32>],
            mem: &mut Cache,
        ) -> i32 {
            if let Some(ans) = mem.get(&(x, y, direction, target, turn_made)) {
                return *ans;
            }

            let mut max_continuation = 0;
            let (dx, dy) = DIRECTIONS[direction];
            let (nx, ny) = (x.wrapping_add(dx), y.wrapping_add(dy));
            if nx < grid.len() && ny < grid[0].len() && grid[nx][ny] == target {
                max_continuation = max_continuation.max(rec(
                    nx,
                    ny,
                    direction,
                    target ^ 0b10,
                    turn_made,
                    grid,
                    mem,
                ));
            }

            if !turn_made {
                let new_direction = (direction + 1) % DIRECTIONS.len();
                let (dx, dy) = DIRECTIONS[new_direction];
                let (nx, ny) = (x.wrapping_add(dx), y.wrapping_add(dy));
                if nx < grid.len() && ny < grid[0].len() && grid[nx][ny] == target {
                    max_continuation = max_continuation.max(rec(
                        nx,
                        ny,
                        new_direction,
                        target ^ 0b10,
                        true,
                        grid,
                        mem,
                    ));
                }
            }

            mem.insert((x, y, direction, target, turn_made), max_continuation + 1);
            max_continuation + 1
        }

        let mut max = 0;
        let mut cache = Cache::new();
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 1 {
                    let mut max_continuation = 0;
                    for (i, (dx, dy)) in DIRECTIONS.into_iter().enumerate() {
                        let (nx, ny) = (row.wrapping_add(dx), col.wrapping_add(dy));

                        if nx < grid.len() && ny < grid[0].len() && grid[nx][ny] == 2 {
                            max_continuation =
                                max_continuation.max(rec(nx, ny, i, 0, false, &grid, &mut cache));
                        }
                    }
                    max = max.max(1 + max_continuation);
                }
            }
        }
        max
    }
}

fn main() {
    let test_cases = [(
        vec![
            vec![2, 2, 1, 2, 2],
            vec![2, 0, 2, 2, 0],
            vec![2, 0, 1, 1, 0],
            vec![1, 0, 2, 2, 2],
            vec![2, 0, 0, 2, 2],
        ],
        5,
    )];

    for (grid, exp) in test_cases {
        println!("{}  exp: {exp}", Solution::len_of_v_diagonal(grid));
    }
}
