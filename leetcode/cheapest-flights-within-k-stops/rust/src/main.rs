// https://leetcode.com/problems/cheapest-flights-within-k-stops/description/
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let flight_map: HashMap<i32, Vec<(i32, i32)>> =
            flights.into_iter().fold(HashMap::new(), |mut map, flight| {
                map.entry(flight[0])
                    .or_default()
                    .push((flight[1], flight[2]));
                map
            });
        println!("{flight_map:?}");

        let mut distances = vec![i32::MAX; n as usize];
        distances[src as usize] = 0;
        let mut new_distances = distances.clone();
        let mut jumps = 0;
        while jumps <= k {
            for i in 0..distances.len() {
                if distances[i] != i32::MAX {
                    for &(target, cost) in flight_map.get(&(i as i32)).unwrap_or(&vec![]) {
                        new_distances[target as usize] =
                            new_distances[target as usize].min(distances[i] + cost);
                    }
                }
            }
            std::mem::swap(&mut distances, &mut new_distances);
            // println!("{distances:?}");
            // println!("{new_distances:?}");
            jumps += 1;
        }

        if distances[dst as usize] == i32::MAX {
            -1
        } else {
            distances[dst as usize]
        }
    }
}

fn main() {
    let tests = vec![
        (
            4,
            vec![
                vec![0, 1, 100],
                vec![1, 2, 100],
                vec![2, 0, 100],
                vec![1, 3, 600],
                vec![2, 3, 200],
            ],
            0,
            3,
            1,
        ),
        (
            8,
            vec![
                vec![3, 4, 7],
                vec![6, 2, 2],
                vec![0, 2, 7],
                vec![0, 1, 2],
                vec![1, 7, 8],
                vec![4, 5, 2],
                vec![0, 3, 2],
                vec![7, 0, 6],
                vec![3, 2, 7],
                vec![1, 3, 10],
                vec![1, 5, 1],
                vec![4, 1, 6],
                vec![4, 7, 5],
                vec![5, 7, 10],
            ],
            4,
            3,
            7,
        ),
    ];

    for (n, flights, src, dst, k) in tests {
        println!("{}", Solution::find_cheapest_price(n, flights, src, dst, k));
    }
}
