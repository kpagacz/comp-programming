// https://leetcode.com/problems/count-unguarded-cells-in-the-grid/description/
pub struct Solution;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        const GUARD: i32 = 0;
        const WALL: i32 = 1;
        let mut row_objects = vec![Vec::default(); m as usize];
        let mut col_objects = vec![Vec::default(); n as usize];

        for guard in guards {
            let (grow, gcol) = (guard[0] as usize, guard[1] as usize);
            row_objects[grow].push((gcol, GUARD));
            col_objects[gcol].push((grow, GUARD));
        }
        for wall in walls {
            let (grow, gcol) = (wall[0] as usize, wall[1] as usize);
            row_objects[grow].push((gcol, WALL));
            col_objects[gcol].push((grow, WALL));
        }

        row_objects
            .iter_mut()
            .for_each(|row_objects| row_objects.sort_unstable());
        col_objects
            .iter_mut()
            .for_each(|col_objects| col_objects.sort_unstable());

        let mut answer = 0;
        (0..m as usize).for_each(|row| {
            (0..n as usize).for_each(|col| {
                // In row
                let pp = row_objects[row].partition_point(|&(pos, _)| pos < col);
                if pp < row_objects[row].len() {
                    let next_or_same = row_objects[row][pp];
                    if next_or_same.0 == col || next_or_same.1 == GUARD {
                        return;
                    }
                }
                if pp > 0 {
                    let previous = row_objects[row][pp - 1];
                    if previous.1 == GUARD {
                        return;
                    }
                }

                // In col
                let pp = col_objects[col].partition_point(|&(pos, _)| pos < row);
                if pp < col_objects[col].len() {
                    let next_or_same = col_objects[col][pp];
                    if next_or_same.0 == row || next_or_same.1 == GUARD {
                        return;
                    }
                }
                if pp > 0 {
                    let previous = col_objects[col][pp - 1];
                    if previous.1 == GUARD {
                        return;
                    }
                }

                answer += 1;
            })
        });
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
