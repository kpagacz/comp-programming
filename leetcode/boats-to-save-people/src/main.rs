// https://leetcode.com/problems/boats-to-save-people/description/
pub struct Solution;

impl Solution {
    pub fn num_rescue_boats_stupid(mut people: Vec<i32>, limit: i32) -> i32 {
        use std::collections::BTreeMap;

        people.sort_unstable();
        let mut boats = 0;
        let mut spaces: BTreeMap<i32, i32> = BTreeMap::new();

        for person in people.into_iter().rev() {
            let mut to_remove: Option<i32> = None;
            match spaces.range_mut(person..).next() {
                Some((key, value)) => {
                    *value -= 1;
                    if *value == 0 {
                        to_remove = Some(*key);
                    }
                }
                None => {
                    *spaces.entry(limit - person).or_default() += 1;
                    boats += 1;
                }
            }
            if let Some(to_remove) = to_remove {
                spaces.remove(&to_remove);
            }
        }

        boats
    }

    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut spaces = vec![];

        let mut boats = 0;
        for person in people.into_iter().rev() {
            match spaces.last() {
                Some(space) if *space >= person => {
                    spaces.pop();
                }
                Some(_) | None => {
                    spaces.push(limit - person);
                    boats += 1;
                }
            }
        }
        boats
    }
}

fn main() {
    println!("Hello, world!");
}
