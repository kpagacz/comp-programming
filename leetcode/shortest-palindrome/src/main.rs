// https://leetcode.com/problems/shortest-palindrome/description/
pub struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        // Rolling hash technique
        let prime = 31;
        let mut current_pow = 1;
        let mut hash = 0;
        let mut rev_hash = 0;
        let mut palindrome_end = 0usize;

        for (i, &c) in s.as_bytes().iter().enumerate() {
            hash += c as i32 * current_pow;
            rev_hash = rev_hash * prime + c as i32;

            if hash == rev_hash {
                palindrome_end = i;
            }

            current_pow *= prime;
        }

        let take_from_end = s.len() - (palindrome_end + 1);
        s.chars().rev().take(take_from_end).collect::<String>() + &s
    }

    fn kmp_solution(s: String) -> String {
        fn prefix_function(s: String) -> Vec<usize> {
            let s = s.as_bytes();
            let n: usize = s.len();
            let mut pi: Vec<usize> = vec![0; n];
            for i in 1..n {
                let mut j = pi[i - 1];
                while j > 0 && s[i] != s[j] {
                    j = pi[j - 1];
                }
                if s[i] == s[j] {
                    j += 1
                };
                pi[i] = j;
            }
            pi
        }
        let n = s.len();
        let rs: String = s.clone().chars().rev().collect();
        let tmp: String = s.clone() + "+" + &rs;
        println!("tmp: {}", &tmp);
        let pi = prefix_function(tmp);
        println!("pi: {pi:?}");
        let last = pi.last().unwrap();
        rs[..(n - last)].to_string() + &s
    }
}

fn main() {
    let test_cases = ["aacecaaa"];
    for s in test_cases {
        let s = s.to_owned();
        println!("{}", Solution::kmp_solution(s));
    }
}
