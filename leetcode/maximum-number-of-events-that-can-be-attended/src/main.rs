// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/description/?envType=daily-question&envId=2025-07-07
pub struct Solution;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events
            .into_iter()
            .map(|event| *event.first_chunk::<2>().unwrap())
            .collect::<Vec<_>>();
        events.sort_unstable();
        let mut events = events.into_iter().peekable();

        use std::collections::BinaryHeap;
        let mut ends = BinaryHeap::new();

        let mut answer = 0;
        let mut current_day;

        if let Some([start, end]) = events.next() {
            ends.push(std::cmp::Reverse(end));
            current_day = start;
        } else {
            return answer;
        }

        // The invariant is that ends contains currently active events
        // where currently active means start <= current day
        while let Some(std::cmp::Reverse(end)) = ends.pop() {
            if current_day <= end {
                answer += 1;
                current_day += 1;

                while let Some([start, end]) = events.peek() {
                    if *start <= current_day {
                        ends.push(std::cmp::Reverse(*end));
                        events.next();
                    } else {
                        break;
                    }
                }
            }

            // This is nice
            if ends.is_empty() {
                if let Some([start, end]) = events.next() {
                    ends.push(std::cmp::Reverse(end));
                    current_day = start;
                }
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
