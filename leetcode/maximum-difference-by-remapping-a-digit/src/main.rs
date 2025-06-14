// https://leetcode.com/problems/maximum-difference-by-remapping-a-digit/description/?envType=daily-question&envId=2025-06-14
pub struct Solution;

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        fn num_with_remap(mut num: i32, from: i32, to: i32) -> i32 {
            let mut pow = 1;
            let mut answer = 0;
            while num > 0 {
                let mut m = num % 10;
                if m == from {
                    m = to;
                }
                answer += pow * m;
                pow *= 10;
                num /= 10;
            }
            answer
        }

        let mut min = num;
        let mut max = num;
        for i in 0..=9 {
            for j in 0..=9 {
                let remapped = num_with_remap(num, i, j);
                min = min.min(remapped);
                max = max.max(remapped);
            }
        }

        max - min
    }
}

fn main() {
    println!("Hello, world!");
}
