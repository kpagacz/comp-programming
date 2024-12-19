// https://leetcode.com/problems/max-chunks-to-make-sorted/description/
pub struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut chunks = 0;
        let mut max = i32::MIN;

        for (pos, num) in arr.into_iter().enumerate() {
            max = max.max(num);
            if max == pos as i32 {
                chunks += 1;
            }
        }
        chunks
    }
}

fn main() {
    println!("Hello, world!");
}
