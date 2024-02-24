// https://leetcode.com/problems/find-all-people-with-secret/description/
pub struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let mut knows_secret = vec![false; n as usize];
        knows_secret[0] = true;
        knows_secret[first_person as usize] = true;
        meetings.sort_unstable_by_key(|meeting| meeting[2]);

        let mut last_time = 0;
        let mut infected_in_this_timestamp = BTreeMap::new();
        infected_in_this_timestamp.insert(0, vec![first_person]);
        infected_in_this_timestamp.insert(first_person, vec![0]);

        for meeting in meetings {
            let (p1, p2, time) = (meeting[0], meeting[1], meeting[2]);
            // println!("meeting: {meeting:?}");
            // println!("infected map: {infected_in_this_timestamp:?}");
            if time != last_time {
                // floodfill
                infected_in_this_timestamp.keys().for_each(|&person| {
                    Solution::floodfill(&infected_in_this_timestamp, &mut knows_secret, person)
                });
                infected_in_this_timestamp.clear();
                last_time = time;
                infected_in_this_timestamp.insert(p1, vec![p2]);
                infected_in_this_timestamp.insert(p2, vec![p1]);
            } else {
                infected_in_this_timestamp
                    .entry(p1)
                    .or_insert(vec![])
                    .push(p2);
                infected_in_this_timestamp
                    .entry(p2)
                    .or_insert(vec![])
                    .push(p1);
            }
        }
        // last floodfill
        infected_in_this_timestamp.keys().for_each(|&person| {
            Solution::floodfill(&infected_in_this_timestamp, &mut knows_secret, person)
        });

        knows_secret
            .into_iter()
            .enumerate()
            .filter_map(|(idx, knows)| if knows { Some(idx as i32) } else { None })
            .collect()
    }

    fn floodfill(infected_map: &BTreeMap<i32, Vec<i32>>, infected: &mut [bool], source: i32) {
        if infected[source as usize] {
            infected_map
                .get(&source)
                .unwrap_or(&vec![])
                .iter()
                .for_each(|&target| {
                    if !infected[target as usize] {
                        infected[target as usize] = true;
                        Solution::floodfill(infected_map, infected, target);
                    }
                })
        }
    }
}

fn main() {
    let tests = vec![(6, vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]], 1)];
    for (n, meetings, first_person) in tests {
        println!("{:?}", Solution::find_all_people(n, meetings, first_person));
    }
}
