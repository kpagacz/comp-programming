// https://leetcode.com/problems/can-place-flowers/description/
pub struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut it = 0;
        let mut answer = 0;
        while it < flowerbed.len() {
            if flowerbed[it] != 1 {
                let mut forward = it;
                while forward < flowerbed.len() && flowerbed[forward] != 1 {
                    forward += 1;
                }

                let mut span = forward - it - 1;
                if it == 0 {
                    span += 1;
                }
                if forward == flowerbed.len() {
                    span += 1;
                }
                answer += span / 2;
                it = forward;
            } else {
                it += 1;
            }
        }
        answer >= n as _
    }
}

fn main() {
    let test_cases = [(vec![1, 0, 0, 0, 1], 1)];
    for (flowerbed, n) in test_cases {
        println!("{}", Solution::can_place_flowers(flowerbed, n));
    }
}
