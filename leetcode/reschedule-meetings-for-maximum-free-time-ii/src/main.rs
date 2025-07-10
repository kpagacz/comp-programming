// https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-ii/description/
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut gaps = vec![];
        let mut previous_end = 0;
        for i in 0..start_time.len() {
            gaps.push(start_time[i] - previous_end);
            previous_end = end_time[i];
        }
        gaps.push(event_time - previous_end);
        let mut three_biggest = [0; 3];
        for gap in &gaps {
            if *gap >= three_biggest[0] {
                three_biggest.rotate_right(1);
                three_biggest[0] = *gap;
            } else if *gap >= three_biggest[1] {
                three_biggest[1..].rotate_right(1);
                three_biggest[1] = *gap;
            } else if *gap >= three_biggest[2] {
                three_biggest[2] = *gap;
            }
        }

        let mut answer = 0;
        for i in 0..gaps.len() - 1 {
            let (bigger, smaller) = (gaps[i].max(gaps[i + 1]), gaps[i + 1].min(gaps[i]));
            let meeting_time = end_time[i] - start_time[i];
            let available_gap: i32;
            if bigger != three_biggest[0] {
                available_gap = three_biggest[0];
            } else if smaller != three_biggest[1] {
                available_gap = three_biggest[1];
            } else {
                available_gap = three_biggest[2];
            }

            if available_gap >= meeting_time {
                answer = answer.max(bigger + smaller + meeting_time);
            } else {
                answer = answer.max(bigger + smaller);
            }
        }

        answer
    }
}

fn main() {
    let test_cases = [
        (5, vec![1, 3], vec![2, 5], 2),
        (10, vec![0, 7, 9], vec![1, 8, 10], 7),
        (20, vec![0, 7, 15], vec![1, 8, 18], 14),
        (20, vec![2, 4, 6, 9, 18], vec![4, 6, 9, 16, 20], 4),
        (25, vec![18, 22, 24], vec![22, 23, 25], 18),
    ];
    for (event_time, start_time, end_time, exp) in test_cases {
        println!(
            "{} exp: {exp}",
            Solution::max_free_time(event_time, start_time, end_time)
        );
    }
}
