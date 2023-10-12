// https://leetcode.com/problems/number-of-flowers-in-full-bloom/description/
pub struct Solution {}

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        #[derive(PartialEq, PartialOrd, Eq, Ord)]
        enum Event {
            Start,
            Person(usize),
            End,
        }

        let mut events: Vec<(i32, Event)> = Vec::with_capacity(2 * flowers.len() + people.len());
        flowers.iter().for_each(|flower| {
            events.push((flower[0], Event::Start));
            events.push((flower[1], Event::End));
        });
        people
            .iter()
            .enumerate()
            .for_each(|(id, arrival)| events.push((*arrival, Event::Person(id))));
        events.sort_unstable();

        let mut full_blooms_for_person = vec![0; people.len()];
        let mut blooming_flowers = 0;
        events.iter().for_each(|(_, event)| match event {
            Event::Start => blooming_flowers += 1,
            Event::End => blooming_flowers -= 1,
            Event::Person(id) => full_blooms_for_person[*id] = blooming_flowers,
        });
        full_blooms_for_person
    }

    pub fn full_bloom_flowers2(mut flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut ends = flowers.iter().map(|flower| flower[1]).collect::<Vec<i32>>();
        flowers.sort_unstable();
        ends.sort_unstable();

        people
            .iter()
            .map(|&arrival_time| {
                flowers.partition_point(|flower| flower[0] <= arrival_time) as i32
                    - ends.partition_point(|&end_time| end_time < arrival_time) as i32
            })
            .collect::<Vec<i32>>()
    }
}
fn main() {
    println!("Hello, world!");
}
