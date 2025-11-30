// https://leetcode.com/problems/find-maximum-balanced-xor-subarray-length/description/
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn max_balanced_subarray(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        type State = (i32, i32); // XOR, #evens - #odds
        let mut prefix = HashMap::<State, usize>::new();
        prefix.insert((0, 0), 0);
        let mut max = 0;
        let mut running_xor = 0;
        let mut evens = 0;
        let mut odds = 0;
        for (pos, num) in nums.into_iter().enumerate() {
            running_xor ^= num;
            if num % 2 == 0 {
                evens += 1;
            } else {
                odds += 1;
            }
            if let Some(nums_taken) = prefix.get(&(running_xor, evens - odds)) {
                max = max.max(pos + 1 - nums_taken);
            } else {
                prefix.insert((running_xor, evens - odds), pos + 1);
            }
        }
        max as _
    }
}
fn main() {
    println!("Hello, world!");
}
