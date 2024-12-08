// https://leetcode.com/problems/two-best-non-overlapping-events/description/
pub struct Solution;

impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        let mut events_copy = events.clone();
        events_copy.sort_unstable_by_key(|event| event[1]);
        events.sort_unstable_by_key(|event| (event[0], event[1]));
        use std::collections::HashMap;
        let mut suffix_map = HashMap::new();
        let mut suffix_max = 0;
        let mut in_events = events.len() - 1;

        (0..events_copy.len()).rev().for_each(|i| {
            while in_events > 0 && events[in_events][0] > events_copy[i][1] {
                suffix_max = suffix_max.max(events[in_events][2]);
                in_events -= 1;
            }
            suffix_map.insert(events_copy[i][1], suffix_max);
        });

        events
            .into_iter()
            .map(|event| event[2] + suffix_map.get(&event[1]).unwrap())
            .max()
            .unwrap()
    }
}

fn main() {
    let test_cases = [vec![
        vec![10, 83, 53],
        vec![63, 87, 45],
        vec![97, 100, 32],
        vec![51, 61, 16],
    ]];

    for events in test_cases {
        println!("{}", Solution::max_two_events(events));
    }
}
