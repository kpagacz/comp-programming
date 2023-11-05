// https://leetcode.com/problems/find-the-winner-of-an-array-game/description/
pub struct Solution {}

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut current_max = &arr[0];
        let mut current_score = -1;

        for num in &arr {
            if num > &current_max {
                current_score = 1;
                current_max = &num;
            } else {
                current_score += 1;
            }

            if current_score >= k {
                return *current_max;
            }
        }

        *current_max
    }
    pub fn get_winner_stupid(arr: Vec<i32>, k: i32) -> i32 {
        if k as usize >= arr.len() {
            return *arr.iter().max().unwrap();
        }

        let mut monotonic = Vec::with_capacity(arr.len());
        let mut scores = vec![0; arr.len()];

        (0..arr.len()).for_each(|pos| {
            let first = monotonic.first().and_then(|&first| Some(first));
            while !monotonic.is_empty() && arr[pos] > arr[*monotonic.last().unwrap()] {
                let popped = monotonic.pop().unwrap();
                scores[popped] += pos - popped - 1;
            }
            if monotonic.is_empty() {
                if let Some(first) = first {
                    scores[first] += 1;
                }
            }
            monotonic.push(pos);
        });
        scores[0] = scores[0].saturating_sub(1);

        println!("{scores:?}");
        let max_pos = *monotonic.first().unwrap();
        let k = k as usize;
        for i in 0..max_pos {
            if scores[i] >= k {
                return arr[i];
            }
        }

        arr[max_pos]
    }
}
fn main() {
    println!("{}", Solution::get_winner(vec![1, 25, 35, 42, 68, 70], 2));
    println!("{}", Solution::get_winner(vec![2, 1, 3, 5, 4, 6, 7], 5));
}
