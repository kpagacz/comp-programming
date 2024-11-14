// https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/description/
pub struct Solution;

impl Solution {
    pub fn minimized_maximum(mut n: i32, quantities: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;

        n -= quantities.len() as i32;

        let mut quantities = quantities
            .into_iter()
            .map(|quantity| (quantity, 1, quantity))
            .collect::<BinaryHeap<_>>();

        while n > 0 {
            let (_, stores, total_quantity) = quantities.pop().unwrap();
            if total_quantity % (stores + 1) == 0 {
                quantities.push((total_quantity / (stores + 1), stores + 1, total_quantity));
            } else {
                quantities.push((
                    total_quantity / (stores + 1) + 1,
                    stores + 1,
                    total_quantity,
                ));
            }
            n -= 1;
        }

        quantities.peek().unwrap().0
    }
}

fn main() {
    println!("Hello, world!");
}
