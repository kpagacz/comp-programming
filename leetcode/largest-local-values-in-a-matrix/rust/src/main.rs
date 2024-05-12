// https://leetcode.com/problems/largest-local-values-in-a-matrix/description/
pub struct Solution;

const PIECE: [(i32, i32); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut answer = vec![vec![0; grid[0].len() - 2]; grid.len() - 2];

        for i in 1..grid.len() - 1 {
            for j in 1..grid[0].len() - 1 {
                answer[i - 1][j - 1] = PIECE
                    .iter()
                    .map(|(delta_x, delta_y)| {
                        grid[(i as i32 + delta_x) as usize][(j as i32 + delta_y) as usize]
                    })
                    .max()
                    .unwrap();
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
