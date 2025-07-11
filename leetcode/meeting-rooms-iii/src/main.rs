// https://leetcode.com/problems/meeting-rooms-iii/?envType=daily-question&envId=2025-07-11
struct Solution;

#[allow(dead_code)]
impl Solution {
    // Fucking overflow issue...
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::collections::VecDeque;

        type Event = (usize, i32, i32); // time, type: 0=End 1=Start, length/room

        let mut events: BinaryHeap<std::cmp::Reverse<Event>> = BinaryHeap::new();
        let mut waiting_list = VecDeque::new();

        for meeting in meetings {
            let (start, end) = (meeting[0], meeting[1]);
            let length = end - start;

            events.push(std::cmp::Reverse((start as usize, 1, length)));
        }

        let mut rooms = BinaryHeap::new();
        (0..n).map(std::cmp::Reverse).for_each(|i| rooms.push(i));

        let mut visited = [0; 101];
        while let Some(std::cmp::Reverse(event)) = events.pop() {
            // println!();
            // println!("event: {event:?}");
            // println!("events: {events:?}");
            // println!("waiting list: {waiting_list:?}");
            match event.1 {
                0 => {
                    // End
                    let (time, _, room) = event;
                    rooms.push(std::cmp::Reverse(room));
                    if let Some((_, length)) = waiting_list.pop_front() {
                        let free_room = rooms.pop().unwrap().0;
                        events.push(std::cmp::Reverse((time + length as usize, 0, free_room)));
                        visited[free_room as usize] += 1;
                    }
                }
                1 => {
                    // Start
                    let (start, _, length) = event;
                    if let Some(std::cmp::Reverse(free_room)) = rooms.pop() {
                        events.push(std::cmp::Reverse((start + length as usize, 0, free_room)));
                        visited[free_room as usize] += 1;
                    } else {
                        waiting_list.push_back((start, length));
                    }
                }
                _ => unreachable!("Unknown event type"),
            }
        }

        // println!("visited: {visited:?}");
        visited
            .into_iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, freq)| *freq)
            .unwrap()
            .0 as _
    }

    pub fn most_booked2(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;

        let mut events: BinaryHeap<std::cmp::Reverse<(usize, i32)>> = BinaryHeap::new();
        meetings.sort_unstable_by_key(|event| event[0]);

        let mut rooms = BinaryHeap::new();
        (0..n).map(std::cmp::Reverse).for_each(|i| rooms.push(i));

        let mut visited = [0; 101];
        for meeting in meetings {
            let (start, end) = (meeting[0], meeting[1]);
            let length = (end - start) as usize;

            while let Some(std::cmp::Reverse(end)) = events.peek() {
                if end.0 <= start as usize {
                    let std::cmp::Reverse((_, room)) = events.pop().unwrap();
                    rooms.push(std::cmp::Reverse(room));
                } else {
                    break;
                }
            }

            if let Some(std::cmp::Reverse(free_room)) = rooms.pop() {
                visited[free_room as usize] += 1;
                events.push(std::cmp::Reverse((start as usize + length, free_room)));
            } else {
                let std::cmp::Reverse((time, room)) = events.pop().unwrap();
                visited[room as usize] += 1;
                events.push(std::cmp::Reverse((time + length, room)));
            }
        }

        // println!("visited: {visited:?}");
        visited
            .into_iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, freq)| *freq)
            .unwrap()
            .0 as _
    }
}

fn main() {
    let test_cases = [
        (2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]], 0),
        (
            3,
            vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]],
            1,
        ),
        (
            2,
            vec![vec![0, 10], vec![1, 2], vec![12, 14], vec![13, 15]],
            0,
        ),
        (
            2,
            vec![
                vec![0, 10],
                vec![1, 5],
                vec![2, 7],
                vec![3, 4],
                vec![8, 11],
                vec![9, 12],
            ],
            0,
        ),
        (
            4,
            vec![
                vec![48, 49],
                vec![22, 30],
                vec![13, 31],
                vec![31, 46],
                vec![37, 46],
                vec![32, 36],
                vec![25, 36],
                vec![49, 50],
                vec![24, 34],
                vec![6, 41],
            ],
            0,
        ),
    ];
    for (n, meetings, exp) in test_cases {
        println!("{} exp: {exp}", Solution::most_booked(n, meetings));
    }
}
