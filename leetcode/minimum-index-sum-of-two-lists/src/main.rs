// https://leetcode.com/problems/minimum-index-sum-of-two-lists/description/
pub struct Solution;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let list2 = list2
            .into_iter()
            .enumerate()
            .fold(HashMap::new(), |mut map, (pos, word)| {
                map.insert(word, pos);
                map
            });

        let min_sum = list1
            .iter()
            .enumerate()
            .filter_map(|(pos, word)| {
                let other_pos = list2.get(word)?;
                Some(pos + other_pos)
            })
            .min()
            .unwrap();

        list1
            .into_iter()
            .enumerate()
            .filter_map(|(pos, word)| {
                let other_pos = list2.get(&word)?;

                if other_pos + pos == min_sum {
                    Some(word)
                } else {
                    None
                }
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
