// https://leetcode.com/problems/arranging-coins/description/
pub struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let coins = |rows: i64| rows * (rows + 1) / 2;
        let n = n as i64;

        let (mut left, mut right) = (1, i32::MAX as i64 / 2);

        while left < right {
            let mid = left + (right - left) / 2;

            match (coins(mid - 1) >= n, coins(mid) >= n) {
                (true, true) => right = mid,
                (false, false) => left = mid + 1,
                (false, true) => {
                    if coins(mid) == n {
                        return mid as _;
                    } else {
                        return mid as i32 - 1;
                    }
                }
                (true, false) => unreachable!(),
            }
        }

        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}
