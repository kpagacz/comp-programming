use std::thread::current;

// https://leetcode.com/problems/count-pairs-of-points-with-distance-k/description/
pub struct Solution {}

impl Solution {
    pub fn count_pairs(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut answer = 0;
        use std::collections::HashMap;
        let mut coordinates_map = HashMap::new();

        for pair in &coordinates {
            for complement in 0..=k {
                answer += *coordinates_map
                    .get(&(pair[0] ^ complement, pair[1] ^ (k - complement)))
                    .unwrap_or(&0);
            }

            *coordinates_map.entry((pair[0], pair[1])).or_insert(0) += 1;
        }
        answer
    }

    pub fn count_pairs_somewhat_slow(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut answer = 0;
        use std::collections::HashMap;
        let mut xs = HashMap::new();
        for i in 0..coordinates.len() {
            let current_coordinate = &coordinates[i];

            for complement in 0..=k {
                let other_x = current_coordinate[0] ^ complement;
                if let Some(matching_coordinates) = xs.get(&other_x) {
                    for &matching_coordinate in matching_coordinates {
                        let matching_coordinate: &Vec<i32> = &coordinates[matching_coordinate];
                        if (current_coordinate[1] ^ matching_coordinate[1]) == k - complement {
                            answer += 1;
                        }
                    }
                }
            }

            xs.entry(current_coordinate[0])
                .and_modify(|indices: &mut Vec<usize>| indices.push(i))
                .or_insert(vec![i]);
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
