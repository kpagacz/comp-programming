// https://leetcode.com/problems/combination-sum-ii/description/
pub struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::collections::hash_map::Iter;
        use std::collections::HashMap;
        type Solutions = Vec<Vec<i32>>;
        fn rec(
            target: i32,
            current_sum: i32,
            current: &mut Vec<i32>,
            solutions: &mut Solutions,
            mut frequencies_iter: Iter<i32, i32>,
        ) {
            if target == current_sum {
                solutions.push(current.clone());
                return;
            }
            if let Some((&key, &value)) = frequencies_iter.next() {
                for i in 0..=value {
                    if current_sum + i * key <= target {
                        for _ in 0..i {
                            current.push(key);
                        }
                        rec(
                            target,
                            current_sum + i * key,
                            current,
                            solutions,
                            frequencies_iter.clone(),
                        );
                        for _ in 0..i {
                            current.pop();
                        }
                    }
                }
            }
        }

        let mut solutions = Solutions::new();
        let mut current = vec![];
        let frequencies =
            candidates
                .into_iter()
                .fold(HashMap::<i32, i32>::new(), |mut map, num| {
                    *map.entry(num).or_insert(0) += 1;
                    map
                });
        rec(target, 0, &mut current, &mut solutions, frequencies.iter());

        solutions
    }
}

fn main() {
    println!("Hello, world!");
}
