// https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner/description/
pub struct Solution;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::collections::HashSet;

        type State = (usize, usize); // removed blocks, x, y
        let mut seen: HashSet<State> = HashSet::default();
        let mut pq = BinaryHeap::default();
        pq.push((0, 0usize, 0usize));

        let m = grid.len();
        let n = grid[0].len();
        while let Some((removed_blocks, row, col)) = pq.pop() {
            if row == m - 1 && col == n - 1 {
                return -removed_blocks;
            }
            if seen.contains(&(row, col)) {
                continue;
            }
            seen.insert((row, col));

            Solution::get_neighbours(row, col, m, n)
                .into_iter()
                .filter(|(new_row, new_col)| !seen.contains(&(*new_row, *new_col)))
                .for_each(|(new_row, new_col)| {
                    if grid[new_row][new_col] == 1 {
                        pq.push((removed_blocks - 1, new_row, new_col));
                    } else {
                        pq.push((removed_blocks, new_row, new_col));
                    }
                });
        }
        unreachable!()
    }

    fn get_neighbours(x: usize, y: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
        let mut ns = vec![];
        if x > 0 {
            ns.push((x - 1, y));
        }
        if y > 0 {
            ns.push((x, y - 1));
        }
        if x + 1 < m {
            ns.push((x + 1, y));
        }
        if y + 1 < n {
            ns.push((x, y + 1));
        }

        ns
    }

    // This is something people call 0-1 BFS. Which is a BFS with 0 or 1 edge weight
    pub fn minimum_obstacles_better(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid.first().map_or(0, Vec::len);
        let mut answer = 0;
        let mut queue1 = Vec::default();
        let mut queue2 = Vec::default();
        let mut node = (0usize, 0usize);

        loop {
            for (new_row, new_col) in [
                (node.0.wrapping_sub(1), node.1),
                (node.0 + 1, node.1),
                (node.0, node.1.wrapping_sub(1)),
                (node.0, node.1 + 1),
            ] {
                if new_row == m - 1 && new_col == n - 1 {
                    return answer;
                }

                if let Some(cell) = grid.get_mut(new_row).and_then(|row| row.get_mut(new_col)) {
                    let queue = match *cell {
                        0 => &mut queue1,
                        1 => &mut queue2,
                        _ => continue,
                    };

                    *cell = 2;
                    queue.push((new_row, new_col));
                }
            }

            if queue1.is_empty() {
                std::mem::swap(&mut queue1, &mut queue2);
                answer += 1;
            }
            node = queue1.pop().unwrap();
        }
    }
}

fn main() {
    println!("Hello, world!");
}
