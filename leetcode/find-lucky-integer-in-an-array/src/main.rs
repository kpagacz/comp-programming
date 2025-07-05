// https://leetcode.com/problems/find-lucky-integer-in-an-array/description/?envType=daily-question&envId=2025-07-05
pub struct Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut freq = vec![0; 501];

        for num in arr {
            freq[num as usize] += 1;
        }

        let mut max = -1;
        for (num, freq) in freq.into_iter().enumerate().skip(1) {
            if num as i32 == freq {
                max = freq;
            }
        }
        max
    }
}

fn main() {
    println!("Hello, world!");
}
