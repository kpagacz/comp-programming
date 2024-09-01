// https://leetcode.com/problems/convert-1d-array-into-2d-array/description/
pub struct Solution;

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let m = m as usize;
        if m * n != original.len() {
            return vec![];
        }
        let mut answer = Vec::with_capacity(m * n);
        let mut original = original.into_iter();
        for _ in 0..m {
            answer.push((&mut original).take(n).collect::<Vec<_>>());
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
