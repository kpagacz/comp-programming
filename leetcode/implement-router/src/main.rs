// https://leetcode.com/problems/implement-router/description/?envType=daily-question&envId=2025-09-20
use std::collections::{HashMap, HashSet, VecDeque};
struct Router {
    queue: VecDeque<[i32; 3]>,
    packets: HashSet<[i32; 3]>,
    memory_limit: usize,
    destination_packets: HashMap<i32, VecDeque<i32>>,
}

#[allow(dead_code)]
impl Router {
    fn new(memory_limit: i32) -> Self {
        Self {
            queue: VecDeque::default(),
            packets: HashSet::default(),
            memory_limit: memory_limit as usize,
            destination_packets: HashMap::default(),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        if !self.packets.contains(&[source, destination, timestamp]) {
            self.queue.push_back([source, destination, timestamp]);
            self.packets.insert([source, destination, timestamp]);
            self.destination_packets
                .entry(destination)
                .and_modify(|timestamps| {
                    timestamps.push_back(timestamp);
                })
                .or_insert(VecDeque::from([timestamp]));
            if self.packets.len() > self.memory_limit {
                self.forward_packet();
            }
            true
        } else {
            false
        }
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        if let Some(packet) = self.queue.pop_front() {
            let [_, destination, _] = packet;
            self.packets.remove(&packet);
            self.destination_packets
                .entry(destination)
                .and_modify(|timestamps| {
                    timestamps.pop_front();
                });
            packet.to_vec()
        } else {
            vec![]
        }
    }

    fn get_count(&mut self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        self.destination_packets
            .get(&destination)
            .map(|timestamps| {
                let start_index = timestamps.partition_point(|stamp| *stamp < start_time);
                let end_index = timestamps.partition_point(|stamp| *stamp <= end_time);

                (end_index - start_index) as i32
            })
            .unwrap_or(0)
    }
}

fn main() {
    println!("Hello, world!");
}
