// https://leetcode.com/problems/the-number-of-the-smallest-unoccupied-chair/description/
pub struct Solution;

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        use std::collections::BTreeSet;
        use std::collections::HashMap;
        let mut free_chairs = BTreeSet::from_iter(0..times.len());
        let mut taken_seats: HashMap<usize, usize> = HashMap::default();

        let mut time_points = times
            .into_iter()
            .enumerate()
            .flat_map(|(pos, time)| vec![(time[0], 1, pos), (time[1], 0, pos)])
            .collect::<Vec<_>>();
        time_points.sort_unstable();

        let target_friend = target_friend as usize;
        for (_, event_type, person) in time_points {
            if person == target_friend {
                return *free_chairs.first().unwrap() as _;
            }
            if event_type == 0 {
                free_chairs.insert(*taken_seats.get(&person).unwrap());
            } else {
                taken_seats.insert(person, free_chairs.pop_first().unwrap());
            }
        }
        unreachable!()
    }

    pub fn smallest_chair_with_heaps(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        use std::collections::BinaryHeap;

        let mut times = (0..).zip(times).collect::<Vec<_>>();
        times.sort_unstable_by_key(|(_, v)| v[0]);
        let mut departures: BinaryHeap<(i32, i32)> = BinaryHeap::default();
        let mut free_chairs: BinaryHeap<i32> = BinaryHeap::default();

        for (person, v) in times {
            while !departures.is_empty() && -departures.peek().unwrap().0 <= v[0] {
                let (_, freed_chair) = departures.pop().unwrap();
                free_chairs.push(-freed_chair);
            }

            if person == target_friend {
                return -free_chairs.pop().unwrap_or(-(departures.len() as i32));
            }

            if free_chairs.is_empty() {
                departures.push((-v[1], departures.len() as i32));
            } else {
                departures.push((-v[1], -free_chairs.pop().unwrap()));
            }
        }

        unreachable!()
    }
}

fn main() {
    let test_cases = [(
        vec![
            vec![18, 19],
            vec![10, 11],
            vec![21, 22],
            vec![5, 6],
            vec![2, 3],
            vec![6, 7],
            vec![43, 44],
            vec![48, 49],
            vec![53, 54],
            vec![12, 13],
            vec![20, 21],
            vec![34, 35],
            vec![17, 18],
            vec![1, 2],
            vec![35, 36],
            vec![16, 17],
            vec![9, 10],
            vec![14, 15],
            vec![25, 26],
            vec![37, 38],
            vec![30, 31],
            vec![50, 51],
            vec![22, 23],
            vec![3, 4],
            vec![27, 28],
            vec![29, 30],
            vec![33, 34],
            vec![39, 40],
            vec![49, 50],
            vec![15, 16],
            vec![4, 5],
            vec![46, 47],
            vec![51, 52],
            vec![32, 33],
            vec![11, 12],
            vec![28, 29],
            vec![47, 48],
            vec![36, 37],
            vec![40, 41],
            vec![42, 43],
            vec![52, 53],
            vec![41, 42],
            vec![31, 32],
            vec![23, 24],
            vec![8, 9],
            vec![19, 20],
            vec![24, 25],
            vec![26, 27],
            vec![45, 46],
            vec![44, 45],
            vec![7, 8],
            vec![13, 14],
            vec![38, 39],
        ],
        8,
    )];

    for (times, target_friend) in test_cases {
        println!(
            "{}",
            Solution::smallest_chair_with_heaps(times, target_friend)
        );
    }
}
