// https://leetcode.com/problems/merge-intervals/description/
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable();
        let mut answer = vec![];
        let mut last_interval_end = intervals[0][1];
        let mut last_start = intervals[0][0];

        for interval in intervals {
            let (new_start, new_end) = (interval[0], interval[1]);

            if new_start > last_interval_end {
                answer.push(vec![last_start, last_interval_end]);
                last_start = new_start;
                last_interval_end = new_end;
            } else {
                last_interval_end = last_interval_end.max(new_end);
            }
        }
        answer.push(vec![last_start, last_interval_end]);

        answer
    }
}
fn main() {
    let test_cases = [
        (vec![vec![1, 3], vec![4, 5]], vec![vec![1, 3], vec![4, 5]]),
        (vec![vec![1, 3], vec![3, 5]], vec![vec![1, 5]]),
        (vec![vec![1, 5], vec![3, 5]], vec![vec![1, 5]]),
        (vec![vec![1, 3], vec![2, 5]], vec![vec![1, 5]]),
    ];
    for (intervals, exp) in test_cases {
        println!("{:?}   exp:{exp:?}", Solution::merge(intervals));
    }
}
