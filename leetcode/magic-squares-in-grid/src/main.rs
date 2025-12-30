// https://leetcode.com/problems/magic-squares-in-grid/description/
struct Solution;

fn magic(g: &[Vec<i32>], i0: usize, j0: usize) -> bool {
    let mut s = [false; 9];
    for i in 0..3 {
        for j in 0..3 {
            let k = (g[i0 + i][j0 + j] - 1) as usize;
            if *s.get(k).unwrap_or(&true) {
                return false;
            }
            s[k] = true
        }
    }
    for k in 0..3 {
        if (0..3).map(|j| g[i0 + k][j0 + j]).sum::<i32>() != 15 {
            return false;
        }
        if (0..3).map(|j| g[i0 + j][j0 + k]).sum::<i32>() != 15 {
            return false;
        }
    }
    if (0..3).map(|i| g[i0 + i][j0 + i]).sum::<i32>() != 15 {
        return false;
    }
    if (0..3).map(|i| g[i0 + i][j0 + 2 - i]).sum::<i32>() != 15 {
        return false;
    }
    true
}

#[allow(dead_code)]
impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut total = 0;
        for i in 0..grid.len().saturating_sub(2) {
            for j in 0..grid[0].len().saturating_sub(2) {
                if magic(&grid, i, j) {
                    total += 1
                }
            }
        }
        total
    }
}

fn main() {
    println!("Hello, world!");
}
