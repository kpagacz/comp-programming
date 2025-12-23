// https://leetcode.com/problems/two-best-non-overlapping-events/description/
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_two_events_probably_smart(mut events: Vec<Vec<i32>>) -> i32 {
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

    // DP with time compressions
    // This is a newer solution by me
    // I am unsure whether I came up with the above myself
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        let mut times = vec![];
        for event in &events {
            times.push(event[0]);
            times.push(event[1]);
        }
        times.sort_unstable();
        times.dedup();
        use std::collections::HashMap;
        let times_map: HashMap<i32, usize> =
            times
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut map, (pos, time)| {
                    map.insert(*time, pos);
                    map
                });

        let mut dp = vec![vec![0; 3]; times.len() + 1]; // dp[i][j] = max value at time
                                                        // i with j events attended
        events.sort_unstable_by_key(|event| (event[1], event[0]));
        let mut last_pos = 0usize;
        for event in events {
            let (start_time, end_time, value) = (event[0], event[1], event[2]);
            // Add one because our dp contains time 0 at the beginning
            let start_pos = 1 + *times_map
                .get(&start_time)
                .expect("Start time is in the map");
            let end_pos = 1 + *times_map.get(&end_time).expect("End time is in the map");

            // Fill skipped indices
            for pos in last_pos + 1..=start_pos {
                for i in 0..3 {
                    dp[pos][i] = dp[pos][i].max(dp[pos - 1][i]);
                }
            }
            last_pos = start_pos;

            // Do the actual dp
            dp[end_pos][1] = dp[end_pos][1].max(dp[start_pos - 1][0] + value);
            dp[end_pos][2] = dp[end_pos][2].max(dp[start_pos - 1][1] + value);
        }
        for pos in last_pos + 1..dp.len() {
            for i in 0..3 {
                dp[pos][i] = dp[pos][i].max(dp[pos - 1][i]);
            }
        }
        dp[dp.len() - 1][1].max(dp[dp.len() - 1][2])
    }
}

fn main() {
    let test_cases = [
        (
            vec![
                vec![10, 83, 53],
                vec![63, 87, 45],
                vec![97, 100, 32],
                vec![51, 61, 16],
            ],
            85,
        ),
        (vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]], 4),
        (vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]], 5),
        (vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]], 8),
    ];

    for (events, exp) in test_cases {
        println!("{} exp: {exp}", Solution::max_two_events(events));
    }
}
