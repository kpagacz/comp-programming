// https://leetcode.com/problems/defuse-the-bomb/description/
pub struct Solution;

impl Solution {
    pub fn decrypt(mut code: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            code.iter_mut().for_each(|num| *num = 0);
            code
        } else if k > 0 {
            let k = k as usize;
            let mut answer = vec![0; code.len()];
            answer.iter_mut().enumerate().for_each(|(pos, num)| {
                let sum = code.iter().cycle().skip(pos + 1).take(k).sum::<i32>();
                *num = sum;
            });
            answer
        } else {
            let k = (-k) as usize;
            let n = code.len();
            let mut answer = vec![0; code.len()];
            answer.iter_mut().enumerate().for_each(|(pos, num)| {
                let sum = code.iter().rev().cycle().skip(n - pos).take(k).sum::<i32>();
                *num = sum;
            });
            answer
        }
    }
}

fn main() {
    let test_cases = [
        (vec![5, 7, 1, 4], 3),
        (vec![1, 2, 3, 4], 0),
        (vec![2, 4, 9, 3], -2),
    ];
    for (code, k) in test_cases {
        println!("{:?}", Solution::decrypt(code, k));
    }
}
