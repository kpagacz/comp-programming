// https://leetcode.com/problems/insert-interval/description/
pub struct Solution;

impl Solution {
    pub fn insert_bad(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        let mut answer = Vec::with_capacity(intervals.len() + 1);

        let mut last_start = intervals[0][0];
        let mut last_end = intervals[0][1];

        for interval in &intervals[1..] {
            let (start, end) = (interval[0], interval[1]);
            if start <= last_end {
                println!("elongate for: {start} {end} {last_start} {last_end}");
                last_end = last_end.max(end);
            } else {
                println!("finish for: {start} {end} {last_start} {last_end}");
                answer.push(vec![last_start, last_end]);
                last_start = start;
                last_end = end;
            }
        }
        answer.push(vec![last_start, last_end]);

        answer
    }

    pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let l = intervals.partition_point(|interval| new_interval[0] > interval[1]);
        let r = intervals.partition_point(|interval| new_interval[1] >= interval[0]);

        if l < intervals.len() {
            new_interval[0] = new_interval[0].min(intervals[l][0]);
        }
        if r > 1 {
            new_interval[1] = new_interval[1].max(intervals[r - 1][1]);
        }

        intervals.splice(l..r, [new_interval]);
        intervals
    }
}

fn main() {
    println!("Hello, world!");
}
