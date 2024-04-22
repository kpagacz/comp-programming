// https://leetcode.com/problems/open-the-lock/description
pub struct Solution;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        type Lock = (u8, u8, u8, u8);
        let chars = target.as_bytes();
        let target = (
            chars[0] - b'0',
            chars[1] - b'0',
            chars[2] - b'0',
            chars[3] - b'0',
        );
        use std::collections::HashSet;
        let mut deadends: HashSet<Lock> =
            deadends.into_iter().fold(HashSet::new(), |mut set, item| {
                let item = item.as_bytes();
                let item = (
                    item[0] - b'0',
                    item[1] - b'0',
                    item[2] - b'0',
                    item[3] - b'0',
                );
                set.insert(item);
                set
            });
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut queue = BinaryHeap::from([(Reverse(0), (0u8, 0u8, 0u8, 0u8))]);

        while let Some(top) = queue.pop() {
            if deadends.contains(&top.1) {
                continue;
            }
            if top.1 == target {
                return top.0 .0;
            }
            deadends.insert(top.1);

            for first in [top.1 .0 + 9, top.1 .0 + 11] {
                let new_lock = (first % 10, top.1 .1, top.1 .2, top.1 .3);
                if !deadends.contains(&new_lock) {
                    queue.push((Reverse(top.0 .0 + 1), new_lock));
                }
            }
            for second in [top.1 .1 + 9, top.1 .1 + 11] {
                let new_lock = (top.1 .0, second % 10, top.1 .2, top.1 .3);
                if !deadends.contains(&new_lock) {
                    queue.push((Reverse(top.0 .0 + 1), new_lock));
                }
            }
            for third in [top.1 .2 + 9, top.1 .2 + 11] {
                let new_lock = (top.1 .0, top.1 .1, third % 10, top.1 .3);
                if !deadends.contains(&new_lock) {
                    queue.push((Reverse(top.0 .0 + 1), new_lock));
                }
            }
            for fourth in [top.1 .3 + 9, top.1 .3 + 11] {
                let new_lock = (top.1 .0, top.1 .1, top.1 .2, fourth % 10);
                if !deadends.contains(&new_lock) {
                    queue.push((Reverse(top.0 .0 + 1), new_lock));
                }
            }
        }

        -1
    }
}

fn main() {
    println!("Hello, world!");
}
