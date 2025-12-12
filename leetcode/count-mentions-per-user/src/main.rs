// https://leetcode.com/problems/count-mentions-per-user/description/?envType=daily-question&envId=2025-12-12
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
        enum Event {
            Offline(usize),
            Online(usize),
            Message(String),
        }

        let n = number_of_users as usize;
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut pq = BinaryHeap::new();

        for event in events {
            let timestamp = event[1].parse::<usize>().unwrap();
            match event[0].as_str() {
                "MESSAGE" => {
                    pq.push(Reverse((timestamp, Event::Message(event[2].clone()))));
                }
                "OFFLINE" => {
                    let user = event[2].parse::<usize>().unwrap();
                    pq.push(Reverse((timestamp, Event::Offline(user))));
                    pq.push(Reverse((timestamp + 60, Event::Online(user))));
                }
                _ => unreachable!("No other types supported"),
            }
        }

        let mut mentions = vec![0; n];
        use std::collections::HashSet;
        let mut online: HashSet<usize> = HashSet::from_iter(0..n);
        while let Some(Reverse((_, event))) = pq.pop() {
            match event {
                Event::Offline(user) => {
                    online.remove(&user);
                }
                Event::Online(user) => {
                    online.insert(user);
                }
                Event::Message(ids) => match ids.as_str() {
                    "ALL" => {
                        for item in &mut mentions {
                            *item += 1;
                        }
                    }
                    "HERE" => {
                        for user in &online {
                            mentions[*user] += 1;
                        }
                    }
                    ids => {
                        for id in ids.split_whitespace() {
                            let id = id.strip_prefix("id").unwrap().parse::<usize>().unwrap();
                            mentions[id] += 1;
                        }
                    }
                },
            }
        }
        mentions
    }
}

fn main() {
    println!("Hello, world!");
}
