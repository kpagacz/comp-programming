// https://leetcode.com/problems/minimum-number-of-seconds-to-make-mountain-height-zero/description/?envType=daily-question&envId=2026-03-13
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_number_of_seconds(mut mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        use std::collections::BinaryHeap;
        let mut pq = BinaryHeap::from_iter(
            worker_times
                .into_iter()
                .map(|time| (time as i64, 0, time as i64))
                .map(std::cmp::Reverse),
        ); // (total_worktime, times worked, time_per_work)
        let mut answer = 0i64;
        while mountain_height > 0 {
            if let Some(std::cmp::Reverse((total_time, times_worked, time_per_unit))) = pq.pop() {
                answer = answer.max(total_time);
                mountain_height -= 1;
                let times_worked = times_worked + 1;
                let next_total_time = total_time + (times_worked + 1) * time_per_unit;
                pq.push(std::cmp::Reverse((
                    next_total_time,
                    times_worked,
                    time_per_unit,
                )));
            } else {
                unreachable!("There should always be a worker")
            }
        }
        answer
    }
}

fn main() {
    let test_cases = [(4, vec![2, 1, 1], 3), (10, vec![3, 2, 2, 4], 12)];
    for (mountain_height, worker_times, exp) in test_cases {
        println!(
            "{} exp: {exp}",
            Solution::min_number_of_seconds(mountain_height, worker_times)
        );
    }
}
