// https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/description/
pub struct Solution;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let nums = HashSet::<i32>::from_iter(arr.iter().cloned());
        let mut max_len = 0;
        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                let mut previous = arr[i];
                let mut current = arr[j];
                let mut curr_len = 0;

                while nums.contains(&(previous + current)) {
                    let old_current = current;
                    current += previous;
                    previous = old_current;
                    curr_len += 1;
                }

                max_len = max_len.max(if curr_len == 0 { 0 } else { 2 + curr_len });
            }
        }
        max_len
    }

    pub fn len_longest_optimized(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let nums = HashSet::<i32>::from_iter(arr.iter().cloned());
        let mut dp = HashMap::new();

        let mut max_len = 0;
        for i in 0..arr.len() {
            for j in 0..i {
                let third = arr[i];
                let second = arr[j];
                let look = third - second;

                if look < second && nums.contains(&look) {
                    let len = *dp.get(&(look, second)).unwrap_or(&2) + 1;
                    dp.insert((second, third), len);
                    max_len = max_len.max(len);
                }
            }
        }
        max_len
    }
}

fn main() {
    println!("Hello, world!");
}
