// https://leetcode.com/problems/ugly-number-ii/description/
pub struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        use std::collections::HashSet;

        let mut visited = HashSet::new();
        use std::collections::BinaryHeap;
        let mut pq = BinaryHeap::new();
        use std::cmp::Reverse;
        pq.push(Reverse(1i64));

        let mut counter = 0;
        loop {
            if let Some(Reverse(num)) = pq.pop() {
                if !visited.contains(&num) {
                    counter += 1;
                    visited.insert(num);
                    if counter == n {
                        return num as _;
                    }

                    pq.push(Reverse(num * 2));
                    pq.push(Reverse(num * 3));
                    pq.push(Reverse(num * 5));
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
