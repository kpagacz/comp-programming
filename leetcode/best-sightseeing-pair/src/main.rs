// https://leetcode.com/problems/best-sightseeing-pair/description/
pub struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let prefix_max: Vec<_> = values
            .iter()
            .enumerate()
            .scan(i32::MIN, |prefix_max, (pos, &num)| {
                let prev = *prefix_max;
                *prefix_max = (*prefix_max).max(pos as i32 + num);
                Some(prev)
            })
            .collect();

        values
            .into_iter()
            .enumerate()
            .map(|(pos, num)| num - pos as i32 + prefix_max[pos])
            .max()
            .unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
