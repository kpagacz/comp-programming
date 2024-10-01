// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/description/
pub struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut rests = vec![0; k as usize];

        for num in arr {
            rests[(((num % k) + k) % k) as usize] += 1;
        }

        for (rest, count) in rests.iter().enumerate().skip(1) {
            let to_k = k as usize - rest;

            if *count != rests[to_k] {
                return false;
            }
        }

        rests[0] % 2 == 0
    }
}

fn main() {
    let test_cases = [
        (vec![1, 2, 3, 4, 5, 6], 7),
        (vec![-1, -1, -1, -1, 2, 2, -2, -2], 3),
    ];

    for (arr, k) in test_cases {
        println!("{}", Solution::can_arrange(arr, k));
    }
}
