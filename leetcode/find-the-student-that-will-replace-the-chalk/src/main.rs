// https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk/description/
pub struct Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let prefix_sum: Vec<_> = chalk
            .into_iter()
            .scan(0i64, |acc, pieces| {
                *acc += pieces as i64;
                Some(*acc)
            })
            .collect();

        let rest = k as i64 % prefix_sum[prefix_sum.len() - 1];
        prefix_sum.partition_point(|&sum| sum <= rest) as _
    }
}

fn main() {
    println!("Hello, world!");
}
