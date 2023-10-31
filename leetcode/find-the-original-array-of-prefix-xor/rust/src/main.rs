// https://leetcode.com/problems/find-the-original-array-of-prefix-xor/



pub struct Solution {}

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut arr = vec![0; pref.len()];
        arr[0] = pref[0];
        (1..pref.len()).for_each(|pos| arr[pos] = pref[pos] ^ pref[pos - 1]);
        arr
    }
    pub fn find_array_smarter(pref: Vec<i32>) -> Vec<i32> {
        pref.iter()
            .scan((0, 0), |(prefix, val), &num| {
                *val = *prefix ^ num;
                *prefix = num;
                Some((*prefix, *val))
            })
            .map(|(_, val)| val)
            .collect()
    }
    pub fn find_array_smarter2(pref: Vec<i32>) -> Vec<i32> {
        use std::iter::once;
        once(pref[0])
            .chain(
                pref.windows(2)
                    .map(|neighbours| neighbours[0] ^ neighbours[1]),
            )
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
