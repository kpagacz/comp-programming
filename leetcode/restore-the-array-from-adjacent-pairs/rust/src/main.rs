// https://leetcode.com/problems/restore-the-array-from-adjacent-pairs/description/
pub struct Solution {}

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;
        let counts = adjacent_pairs.iter().fold(HashMap::new(), |mut map, pair| {
            *map.entry(pair[0]).or_insert(0) += 1;
            *map.entry(pair[1]).or_insert(0) += 1;
            map
        });
        // Exploiting the fact that values are unique
        let edges: Vec<i32> = counts
            .into_iter()
            .filter_map(|(key, count)| if count == 1 { Some(key) } else { None })
            .collect();
        let pairs = adjacent_pairs
            .into_iter()
            .fold(HashMap::new(), |mut map, pair| {
                map.entry(pair[0]).or_insert(vec![]).push(pair[1]);
                map.entry(pair[1]).or_insert(vec![]).push(pair[0]);
                map
            });

        let mut answer = vec![];
        answer.push(edges[0]);
        // println!("{pairs:?}");

        while answer.last().unwrap() != &edges[1] {
            let neighbours = pairs.get(answer.last().unwrap()).unwrap();
            if neighbours.len() == 1 {
                answer.push(neighbours[0]);
            } else {
                let previous = answer[answer.len() - 2];
                if neighbours[0] == previous {
                    answer.push(neighbours[1]);
                } else {
                    answer.push(neighbours[0]);
                }
            }
        }
        answer
    }
}

fn main() {
    let test_cases = vec![vec![vec![2, 1], vec![3, 4], vec![3, 2]]];
    for adjacent_pairs in test_cases {
        println!("{:?}", Solution::restore_array(adjacent_pairs));
    }
}
