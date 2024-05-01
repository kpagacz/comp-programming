// https://leetcode.com/problems/valid-perfect-square/description/
pub struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let (mut left, mut right) = (1, num);

        while left < right {
            let mid = left + (right - left) / 2; // "left sticking"
            if i32::MAX / mid < mid {
                right = mid;
            } else {
                match (mid * mid).cmp(&num) {
                    std::cmp::Ordering::Less => left = mid + 1,
                    std::cmp::Ordering::Equal => return true,
                    std::cmp::Ordering::Greater => right = mid,
                }
            }
        }

        left * left == num
    }
}

fn main() {
    let test_cases = [1, 3, 9, 808201];
    for num in test_cases {
        println!("{}", Solution::is_perfect_square(num));
    }
}
