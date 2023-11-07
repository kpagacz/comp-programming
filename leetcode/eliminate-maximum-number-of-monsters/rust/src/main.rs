// https://leetcode.com/problems/eliminate-maximum-number-of-monsters/description/
pub struct Solution {}

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut times: Vec<i32> = dist
            .into_iter()
            .zip(speed)
            .map(|(d, t)| d as f32 / t as f32)
            .map(|arrival| arrival.ceil() as i32)
            .collect();
        times.sort_unstable();

        let len = times.len() as i32;
        match times
            .into_iter()
            .enumerate()
            .position(|(pos, arrival)| arrival <= pos as i32)
        {
            Some(pos) => pos as i32,
            None => len,
        }
    }
}
fn main() {
    let test_cases = vec![
        (vec![1, 1, 2, 3], vec![1, 1, 1, 1]),
        (vec![3, 2, 4], vec![5, 3, 2]),
        (vec![7, 8], vec![6, 6]),
    ];
    for (dist, speed) in test_cases {
        println!("{}", Solution::eliminate_maximum(dist, speed));
    }
}
