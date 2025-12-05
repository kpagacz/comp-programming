// https://leetcode.com/problems/count-partitions-with-even-sum-difference/description/?envType=daily-question&envId=2025-12-05
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let mut sum = nums.iter().sum::<i32>();
        let mut prefix = 0;
        let mut count = 0;
        let n = nums.len();
        for num in nums.into_iter().take(n - 1) {
            prefix += num;
            sum -= num;
            if (sum - prefix) % 2 == 0 {
                count += 1;
            }
        }

        count
    }
}
fn main() {
    println!("Hello, world!");
}
