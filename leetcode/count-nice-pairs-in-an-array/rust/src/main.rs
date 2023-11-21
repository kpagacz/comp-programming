// https://leetcode.com/problems/count-nice-pairs-in-an-array/
pub struct Solution {}

const MOD: i32 = 10i32.pow(9) + 7;
impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        use std::collections::BTreeMap;
        type Map = BTreeMap<i32, i32>;
        let mut counts = Map::new();
        let mut answer = 0;
        for num in nums {
            let calc = num - Self::rev(num);
            let count = counts.entry(calc).or_insert(0);
            answer = (answer + *count) % MOD;
            *count += 1;
        }

        answer
    }

    fn rev(num: i32) -> i32 {
        num.to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
