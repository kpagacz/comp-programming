// https://leetcode.com/problems/delete-columns-to-make-sorted/description/?envType=daily-question&envId=2025-12-20
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut vecs = vec![vec![]; strs[0].len()];

        for str in strs {
            for (pos, c) in str.as_bytes().iter().enumerate() {
                vecs[pos].push(*c);
            }
        }

        vecs.into_iter()
            .filter(|vec| !vec.windows(2).all(|window| window[0] <= window[1]))
            .count() as _
    }
}

fn main() {
    println!("Hello, world!");
}
