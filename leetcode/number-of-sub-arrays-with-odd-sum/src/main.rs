// https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/description/
pub struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut odd = 0;
        let mut even = 1;
        let mut total = 0;
        let mut current_sum = 0;
        const MOD: i32 = 1_000_000_007;

        for num in arr {
            current_sum += num;
            if current_sum & 1 == 1 {
                total += even;
                odd += 1;
            } else {
                total += odd;
                even += 1;
            }
            total %= MOD;
        }

        total
    }
}

fn main() {
    println!("Hello, world!");
}
