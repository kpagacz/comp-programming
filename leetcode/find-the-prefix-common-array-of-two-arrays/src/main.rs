// https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays/description/
pub struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut freq = HashMap::new();
        let mut count = 0;
        let mut answer = Vec::with_capacity(a.len());
        for i in 0..a.len() {
            *freq.entry(a[i]).or_insert(0) += 1;
            *freq.entry(b[i]).or_insert(0) += 1;
            if *freq.get(&a[i]).unwrap() == 2 {
                count += 1;
            }
            if *freq.get(&b[i]).unwrap() == 2 && b[i] != a[i] {
                count += 1;
            }
            answer.push(count);
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
