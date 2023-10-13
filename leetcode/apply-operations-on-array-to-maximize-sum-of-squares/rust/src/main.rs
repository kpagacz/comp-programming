// https://leetcode.com/problems/apply-operations-on-array-to-maximize-sum-of-squares/description/
pub struct Solution {}

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut bits = vec![0; 32];

        for num in nums {
            for i in 0..32 {
                if ((num >> (31 - i)) & 1) % 2 == 1 {
                    bits[i] += 1;
                }
            }
            println!("{bits:?}");
        }

        let mut squares = 0;
        (0..k).for_each(|_| {
            let mut num = 0i64;
            for i in 0..32 {
                if bits[i] > 0 {
                    num |= 1 << (31 - i);
                    bits[i] -= 1;
                }
            }
            squares = (squares + num * num) % 1_000_000_007;
        });
        squares as i32
    }
}

fn main() {
    println!("Hello, world!");
}
