// https://leetcode.com/problems/maximum-value-at-a-given-index-in-a-bounded-array/

pub struct Solution {}
impl Solution {
    pub fn max_value(n: i32, index: i32, mut max_sum: i32) -> i32 {
        max_sum -= n;
        if max_sum == 0 { return 1; }
        max_sum -= 1;
        if max_sum == 0 { return 2; }
        let mut it = 2;
        let mut needed_for_iteration = 1;
        loop {
            let mut needed_for_next_iteration = needed_for_iteration;
            if index - (it - 1) >= 0 {
                needed_for_next_iteration += 1;
            }
            if index + (it - 1) < n {
                needed_for_next_iteration += 1;
            }

            if needed_for_iteration == needed_for_next_iteration {
                return it + (max_sum as f64 / needed_for_iteration as f64).floor() as i32;
            }

            if max_sum - needed_for_next_iteration >= 0 {
                max_sum -= needed_for_next_iteration;
            } else {
                break;
            }

            needed_for_iteration = needed_for_next_iteration;
            it += 1;
        }

        it
    }
}

fn main() {
    println!("Hello, world!");
}
