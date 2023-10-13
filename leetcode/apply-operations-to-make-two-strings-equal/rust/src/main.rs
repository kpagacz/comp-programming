// https://leetcode.com/problems/apply-operations-to-make-two-strings-equal/description/
pub struct Solution {}
impl Solution {
    pub fn min_operations(s1: String, s2: String, x: i32) -> i32 {
        let diff_ids = s1
            .bytes()
            .zip(s2.bytes())
            .enumerate()
            .filter(|(_, (b1, b2))| b1 != b2)
            .map(|(id, _)| id as i32)
            .collect::<Vec<i32>>();

        if diff_ids.len() == 0 {
            return 0;
        }
        if diff_ids.len() % 2 == 1 {
            return -1;
        }

        use std::collections::HashMap;
        fn top_down(
            left: usize,
            right: usize,
            ids: &Vec<i32>,
            x: i32,
            cache: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if left >= right {
                return 0;
            }
            if let Some(mem) = cache.get(&(left, right)) {
                return *mem;
            }
            let s1 = x.min(ids[right] - ids[left]) + top_down(left + 1, right - 1, ids, x, cache);
            let s2 = x.min(ids[left + 1] - ids[left]) + top_down(left + 2, right, ids, x, cache);
            let s3 = x.min(ids[right] - ids[right - 1])
                + top_down(left, right.saturating_sub(2), ids, x, cache);
            let score = s1.min(s2).min(s3);
            cache.insert((left, right), score);
            score
        }

        let mut cache = HashMap::new();
        top_down(0, diff_ids.len() - 1, &diff_ids, x, &mut cache)
    }
}
fn main() {
    println!("Hello, world!");
}
