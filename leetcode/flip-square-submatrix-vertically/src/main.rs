// https://leetcode.com/problems/flip-square-submatrix-vertically/description/?envType=daily-question&envId=2026-03-21
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let (x, y, k) = (x as usize, y as usize, k as usize);
        for row_delta in 0..k / 2 {
            let origin_row = x + row_delta;
            let dest_row = x + k - 1 - row_delta;
            let (first, second) = grid.split_at_mut(dest_row);
            first[origin_row][y..y + k].swap_with_slice(&mut second[0][y..y + k]);
        }
        grid
    }
}

fn main() {
    println!("Hello, world!");
}
