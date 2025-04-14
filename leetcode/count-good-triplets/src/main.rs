// https://leetcode.com/problems/count-good-triplets/description/?envType=daily-question&envId=2025-04-14
pub struct Solution;

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut answer = 0;
        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                for k in j + 1..arr.len() {
                    if (arr[i] - arr[j]).abs() <= a
                        && (arr[j] - arr[k]).abs() <= b
                        && (arr[k] - arr[i]).abs() <= c
                    {
                        answer += 1;
                    }
                }
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
