// https://leetcode.com/problems/maximum-number-of-tasks-you-can-assign/description/?envType=daily-question&envId=2025-05-01
pub struct Solution;

impl Solution {
    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        fn can_finish_work(tasks: &[i32], workers: &[i32], mut pills: i32, strength: i32) -> bool {
            if tasks.is_empty() {
                return true;
            }
            //println!("{tasks:?} {workers:?}");
            use std::collections::BTreeMap;
            let mut workers: BTreeMap<i32, i32> =
                workers.iter().fold(BTreeMap::new(), |mut map, worker| {
                    map.entry(*worker)
                        .and_modify(|worker| *worker += 1)
                        .or_insert(1);
                    map
                });
            //println!("{workers:?}");
            for &task in tasks.iter().rev() {
                //println!("task: {task}");
                //println!("workers: {workers:?}");
                //println!("range: {}", task - strength);
                //println!("pills: {pills}");
                //println!(
                //     "{:?}",
                //     workers
                //         .range(task - strength..)
                //         .next()
                //         .and_then(|(key, _)| if pills > 0 { Some(key) } else { None })
                // );
                if *workers.last_entry().unwrap().key() >= task {
                    //println!("Top worker handled it");
                    let mut last_entry = workers.last_entry().unwrap();
                    *last_entry.get_mut() -= 1;
                    if last_entry.get() == &0 {
                        workers.pop_last();
                    }
                } else if let Some(&value) = workers
                    .range(task - strength..)
                    .next()
                    .and_then(|(key, _)| if pills > 0 { Some(key) } else { None })
                {
                    let entry = workers.entry(value).or_insert(0);
                    *entry -= 1;
                    if *entry == 0 {
                        workers.remove(&value);
                    }
                    pills -= 1;
                    //println!("Worker handled it with a pill");
                } else {
                    //println!("false");
                    return false;
                }
            }
            //println!("true");
            true
        }
        tasks.sort_unstable();
        workers.sort_unstable();
        let candidates: Vec<_> = (0..=tasks.len()).collect();
        let pp = candidates.partition_point(|&completed_tasks| {
            if completed_tasks > workers.len() {
                return false;
            }
            can_finish_work(
                &tasks[..completed_tasks],
                &workers[workers.len() - completed_tasks..],
                pills,
                strength,
            )
        });
        (pp - 1) as _
    }
}

fn main() {
    let test_cases = [(vec![3, 2, 1], vec![0, 3, 3], 1, 1)];

    for (tasks, workers, pills, strength) in test_cases {
        println!(
            "{}",
            Solution::max_task_assign(tasks, workers, pills, strength)
        );
    }
}
