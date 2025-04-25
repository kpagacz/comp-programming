// https://leetcode.com/problems/count-of-interesting-subarrays/description/?envType=daily-question&envId=2025-04-25
pub struct Solution;

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        use std::collections::HashMap;
        let mut interesting_says = 0i64;
        // How many times this count has been seen
        let mut previous_counts = HashMap::new();
        previous_counts.insert(0, 1);

        let mut cnt = 0;
        for num in nums {
            // Pair up this count with previous counts
            if num % modulo == k {
                cnt += 1;
                cnt %= modulo;
            }
            // current count = cnt
            // target cnt = k
            // cnt - looking = k
            // looking = cnt - k
            // looking = (cnt + modulo - k) % modulo
            let entry = previous_counts
                .entry((cnt + modulo - k) % modulo)
                .or_insert(0);
            interesting_says += *entry;
            previous_counts
                .entry(cnt)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            // println!("{previous_counts:?} {interesting_says}");
        }

        interesting_says
    }
}

fn main() {
    let test_cases = [
        (vec![3, 2, 4], 2, 1),
        (vec![3, 1, 9, 6], 3, 0),
        (vec![11, 12, 21, 31], 10, 1),
    ];
    for (nums, modulo, k) in test_cases {
        println!("{nums:?}");
        println!("{}", Solution::count_interesting_subarrays(nums, modulo, k));
    }
}
