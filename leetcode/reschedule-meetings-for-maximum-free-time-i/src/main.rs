// https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-i/description/?envType=daily-question&envId=2025-07-09
#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_free_time(
        event_time: i32,
        k: i32,
        mut start_time: Vec<i32>,
        mut end_time: Vec<i32>,
    ) -> i32 {
        start_time.push(0);
        end_time.push(0);
        let mut events = (0..start_time.len()).collect::<Vec<_>>();
        events.sort_by_key(|&i| (start_time[i], end_time[i]));

        let gaps = events
            .windows(2)
            .take_while(|&window| start_time[window[1]] <= event_time)
            .map(|window| {
                let (first, second) = (window[0], window[1]);
                start_time[second] - end_time[first]
            })
            .chain(std::iter::once(
                event_time - end_time[*events.last().unwrap()],
            ))
            .collect::<Vec<_>>();
        let mut prefix = vec![0; gaps.len() + 1];
        for i in 1..prefix.len() {
            prefix[i] = prefix[i - 1] + gaps[i - 1];
        }
        let span = (k as usize + 1).min(gaps.len());
        let mut answer = 0;
        for span_end in span..=gaps.len() {
            answer = answer.max(prefix[span_end] - prefix[span_end - span]);
        }
        answer
    }
}

fn main() {
    let test_cases = [
        (5, 1, vec![1, 3], vec![2, 5], 2),
        (10, 1, vec![0, 2, 9], vec![1, 4, 10], 6),
        (5, 2, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5], 0),
        (10, 4, vec![0, 2, 9], vec![1, 4, 10], 6),
        (34, 2, vec![0, 17], vec![14, 19], 18),
    ];

    for (event_time, k, start_time, end_time, exp) in test_cases {
        println!(
            "{} | exp: {exp}",
            Solution::max_free_time(event_time, k, start_time, end_time)
        );
    }
}
