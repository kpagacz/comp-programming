// https://leetcode.com/problems/maximum-manhattan-distance-after-k-changes/description/?envType=daily-question&envId=2025-06-20
pub struct Solution;

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let mut current = [0i32; 4]; // 0 - N, 1 - W, 2 - S, 3 - E
        let mut sum = 0;
        let mut max = 0;

        for c in s.chars() {
            match c {
                'N' => current[0] += 1,
                'S' => current[2] += 1,
                'E' => current[3] += 1,
                'W' => current[1] += 1,
                _ => unreachable!(),
            }

            sum += 1;
            max =
                max.max(2 * k + (current[0] - current[2]).abs() + (current[1] - current[3]).abs());
            max = max.min(sum);
        }

        max
    }
}

fn main() {
    println!("Hello, world!");
}
