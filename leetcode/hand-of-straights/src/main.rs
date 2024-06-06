// https://leetcode.com/problems/hand-of-straights/description/?envType=daily-question&envId=2024-06-06
pub struct Solution;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        use std::collections::HashMap;
        let mut freq = hand.iter().fold(HashMap::new(), |mut map, num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

        let mut keys = freq.keys().map(|num| **num).collect::<Vec<_>>();
        keys.sort_unstable();
        for key in keys {
            while *freq.get(&key).unwrap_or(&0) != 0 {
                for i in key..(key + group_size) {
                    dbg!(i);
                    if let Some(f) = dbg!(freq.get_mut(&i)) {
                        if *f == 0 {
                            return false;
                        } else {
                            *f -= 1;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }
}

fn main() {
    let test_case = [
        (vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3),
        (vec![1, 1, 2, 2, 3, 3], 3),
    ];

    for (hand, group_size) in test_case {
        println!("{}", Solution::is_n_straight_hand(hand, group_size));
    }
}
