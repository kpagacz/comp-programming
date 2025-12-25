// https://leetcode.com/problems/maximize-happiness-of-selected-children/description/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, mut k: i32) -> i64 {
        happiness.sort_unstable_by(|a, b| b.cmp(a));

        let mut it = 0;
        let mut sum = 0;
        while k > 0 {
            sum += 0.max(happiness[it] as i64 - it as i64);
            it += 1;
            k -= 1;
        }

        sum
    }
}

fn main() {
    println!("Hello, world!");
}
