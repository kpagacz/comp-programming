// https://leetcode.com/problems/remove-letter-to-equalize-frequency/
pub struct Solution {}
impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        use std::collections::HashMap;
        let freq = word.as_bytes().iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(*c).or_insert(0) += 1;
            map
        });
        // There's only one letter in the word
        // or all letters occur exactly one time
        if freq.len() == 1 || freq.values().all(|count| count == &1) {
            return true;
        }
        let freq = freq.values().fold(HashMap::new(), |mut map, &count| {
            *map.entry(count).or_insert(0) += 1;
            map
        });

        if freq.len() != 2 {
            return false;
        }

        let mut keys: Vec<i32> = freq.keys().copied().collect();
        keys.sort();

        (keys[1] - keys[0] == 1 && freq.get(&keys[1]).unwrap() == &1)
            || (keys[0] == 1 && freq.get(&1).unwrap() == &1)
    }
}

fn main() {
    println!("Hello, world!");
}
