// https://leetcode.com/problems/strange-printer/
pub struct Solution;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let mut mem = vec![vec![-1; s.len()]; s.len()];

        let s = s.as_bytes();
        let s: Vec<u8> = s
            .iter()
            .enumerate()
            .filter(|(pos, &c)| *pos == 0 || (s[pos - 1] != c))
            .map(|(_, &c)| c)
            .collect();

        fn rec(start: usize, end: usize, mem: &mut [Vec<i32>], s: &[u8]) -> i32 {
            if start > end {
                return 0;
            }

            if mem[start][end] != -1 {
                return mem[start][end];
            }

            let mut min_turns = 1 + rec(start + 1, end, mem, s);

            for mid in start + 1..end {
                if s[start] == s[mid] {
                    let optimized_turns = rec(start, mid - 1, mem, s) + rec(mid + 1, end, mem, s);
                    min_turns = min_turns.min(optimized_turns);
                }
            }

            mem[start][end] = min_turns;
            min_turns
        }

        rec(0, s.len() - 1, &mut mem, &s)
    }
}

fn main() {
    let test_cases = [
        ("aabbcbabcdbaacaacaaa", 8),
        ("aabbcbabdbaaaaaaa", 5),
        ("aaa", 1),
        ("aba", 2),
        ("tbgtgb", 4),
        ("leetcode", 6),
    ];

    for (s, answer) in test_cases {
        println!(
            "Answer: {} should be: {}",
            Solution::strange_printer(s.to_string()),
            answer
        );
    }
}
