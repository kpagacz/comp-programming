// https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/description/?envType=daily-question&envId=2025-02-12
pub struct Solution;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut sums = vec![Vec::new(); 100];

        for num in nums {
            let mut sum = 0;
            let mut rest = num;
            while rest > 0 {
                sum += rest % 10;
                rest /= 10;
            }
            let entry = sums.get_mut(sum as usize).unwrap();
            let pp = entry.partition_point(|&x| num < x);
            entry.insert(pp, num);
            if entry.len() > 2 {
                entry.pop();
            }
        }

        sums.into_iter()
            .filter_map(|value| value.get(0..2).map(|slice| slice.iter().sum::<i32>()))
            .max()
            .unwrap_or(-1)
    }
}

fn main() {
    println!("Hello, world!");
}
