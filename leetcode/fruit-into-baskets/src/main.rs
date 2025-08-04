// https://leetcode.com/problems/fruit-into-baskets/description/?envType=daily-question&envId=2025-08-04
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut left = 0;
        let mut max_span = 0;
        let mut types = HashMap::new();
        for i in 0..fruits.len() {
            types
                .entry(fruits[i])
                .and_modify(|val| *val += 1)
                .or_insert(1);

            while types.keys().len() > 2 {
                types
                    .entry(fruits[left])
                    .and_modify(|val| *val -= 1)
                    .or_insert(0);
                types.retain(|_, val| *val != 0);
                left += 1;
            }

            max_span = max_span.max(i - left + 1);
        }
        max_span as _
    }
}

fn main() {
    let test_cases = [
        (vec![1, 2, 1], 3),
        (vec![0, 1, 2, 2], 3),
        (vec![1, 2, 3, 2, 2], 4),
    ];
    for (fruits, exp) in test_cases {
        println!("{}  exp: {exp}", Solution::total_fruit(fruits));
    }
}
