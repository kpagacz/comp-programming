// https://leetcode.com/problems/delete-columns-to-make-sorted-ii/description/?envType=daily-question&envId=2025-12-21
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut answer = 0;
        let mut current_prefix = vec![vec![]; strs.len()];
        let strs: Vec<_> = strs
            .into_iter()
            .map(|str| str.as_bytes().to_vec())
            .collect();
        for letter in 0..strs[0].len() {
            let mut current_prefix2 = current_prefix.clone();
            for word in 0..strs.len() {
                current_prefix2[word].push(strs[word][letter]);
            }
            if Solution::is_sorted(&current_prefix2) {
                current_prefix = current_prefix2;
            } else {
                answer += 1;
            }
        }
        answer
    }

    fn is_sorted(strs: &[Vec<u8>]) -> bool {
        strs.windows(2).all(|window| window[0] <= window[1])
    }
}

fn main() {
    println!("Hello, world!");
}
