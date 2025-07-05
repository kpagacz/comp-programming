// https://leetcode.com/problems/longest-common-prefix-between-adjacent-strings-after-removals/description/
pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(words: Vec<String>) -> Vec<i32> {
        fn count_prefix(first: &str, second: &str) -> usize {
            first
                .as_bytes()
                .iter()
                .zip(second.as_bytes())
                .position(|(first, second)| first != second)
                .unwrap_or(first.len().min(second.len()))
        }

        let mut longest_prefixes = vec![]; // prefix length, window position
        const LIMIT: usize = 3;
        words.windows(2).enumerate().for_each(|(pos, window)| {
            let (first, second) = (&window[0], &window[1]);
            let common_prefix = count_prefix(first, second);
            longest_prefixes.push((common_prefix, pos));
            longest_prefixes.sort_by_key(|(length, _)| std::cmp::Reverse(*length));
            if longest_prefixes.len() > LIMIT {
                longest_prefixes.pop();
            }
        });

        let mut answer = vec![0i32; words.len()];
        for i in 0..words.len() {
            if i == 0 {
                let longest_adjacent = longest_prefixes
                    .iter()
                    .find(|(_, window_pos)| *window_pos != i)
                    .unwrap_or(&(0, 0))
                    .0;
                answer[i] = longest_adjacent as _;
            } else if i == words.len() - 1 {
                let longest_adjacent = longest_prefixes
                    .iter()
                    .find(|(_, window_pos)| *window_pos != i - 1)
                    .unwrap_or(&(0, 0))
                    .0;
                answer[i] = longest_adjacent as _;
            } else {
                let mut cloned_longest = longest_prefixes.clone();
                cloned_longest.push((count_prefix(&words[i - 1], &words[i + 1]), usize::MAX));
                cloned_longest.sort_by_key(|(length, _)| std::cmp::Reverse(*length));
                let longest_adjacent = cloned_longest
                    .into_iter()
                    .find(|(_, window_pos)| *window_pos != i && *window_pos != i - 1)
                    .unwrap_or((0, 0))
                    .0;
                answer[i] = longest_adjacent as i32;
            }
        }
        answer
    }
}

fn main() {
    let test_cases = [vec!["bbbdd", "eddcc", "dca", "ccdb", "c", "ff", "cbfae"]];
    for words in test_cases {
        println!(
            "{:?}",
            Solution::longest_common_prefix(words.into_iter().map(str::to_owned).collect())
        );
    }
}
