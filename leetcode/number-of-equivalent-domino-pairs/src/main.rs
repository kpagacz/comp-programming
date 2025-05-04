// https://leetcode.com/problems/number-of-equivalent-domino-pairs/description/?envType=daily-question&envId=2025-05-04
pub struct Solution;

impl Solution {
    pub fn num_equiv_domino_pairs(mut dominoes: Vec<Vec<i32>>) -> i32 {
        dominoes
            .iter_mut()
            .for_each(|domino| domino.sort_unstable());
        dominoes.sort_unstable();
        dominoes
            .chunk_by(|a, b| a == b)
            .map(|chunk| {
                let l = chunk.len();
                (l * l - 1) / 2
            })
            .sum::<usize>() as _
    }
}

fn main() {
    println!("Hello, world!");
}
