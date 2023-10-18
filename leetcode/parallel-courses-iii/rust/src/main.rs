// https://leetcode.com/problems/parallel-courses-iii/
pub struct Solution {}

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut destinations = vec![Vec::with_capacity(n); n];
        relations.iter().for_each(|relation| {
            destinations[relation[0] as usize - 1].push(relation[1] as usize - 1)
        });

        let mut mem = vec![-1; n];
        fn find_train_times(
            class_to_take: usize,
            time: &Vec<i32>,
            destinations: &Vec<Vec<usize>>,
            mem: &mut Vec<i32>,
        ) -> i32 {
            if destinations[class_to_take].is_empty() {
                mem[class_to_take] = time[class_to_take];
                return time[class_to_take];
            }
            if mem[class_to_take] != -1 {
                return mem[class_to_take];
            }
            let max = destinations[class_to_take]
                .iter()
                .map(|&next_class| find_train_times(next_class, time, destinations, mem))
                .max()
                .unwrap();
            mem[class_to_take] = time[class_to_take] + max;
            mem[class_to_take]
        }
        (0..n).for_each(|start_class| {
            find_train_times(start_class, &time, &destinations, &mut mem);
        });
        *mem.iter().max().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
