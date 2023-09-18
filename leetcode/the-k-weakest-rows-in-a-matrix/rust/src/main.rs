// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix
pub struct Solution {}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut strengths: Vec<(usize, i32)> = mat
            .iter()
            .map(|row| row.iter().take_while(|el| el == &&1).count() as i32)
            .enumerate()
            .collect();
        strengths.sort_unstable_by(|&a, &b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        strengths
            .iter()
            .take(k as usize)
            .map(|(id, _)| *id as i32)
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
