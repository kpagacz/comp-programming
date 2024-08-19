// https://leetcode.com/problems/2-keys-keyboard/description/
pub struct Solution;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut current = 1;
        let mut answer = 0;
        let mut pasted = 0;

        while current != n {
            if n % current == 0 {
                answer += 2;
                pasted = current;
                current *= 2;
            } else {
                answer += 1;
                current += pasted;
            }
        }

        answer
    }
}

fn main() {
    let test_cases = [1, 2, 3, 4, 5, 6, 7, 17, 21];

    for n in test_cases {
        println!("{}", Solution::min_steps(n));
    }
}
