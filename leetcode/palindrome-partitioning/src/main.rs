// https://leetcode.com/problems/palindrome-partitioning/description/
pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn is_palindrome(s: &str) -> bool {
            (!s.is_empty()) && s.chars().eq(s.chars().rev())
        }

        fn rec<'a: 'b, 'b>(
            rest: &'a str,
            palindromes_so_far: &mut Vec<&'b str>,
            res: &mut Vec<Vec<String>>,
        ) {
            if rest.is_empty() {
                res.push(
                    palindromes_so_far
                        .iter()
                        .map(|&s| s.to_owned())
                        .collect::<Vec<_>>(),
                );
            } else {
                for i in 1..=rest.len() {
                    let (s, rest) = rest.split_at(i);
                    if is_palindrome(s) {
                        palindromes_so_far.push(s);
                        rec(rest, palindromes_so_far, res);
                        palindromes_so_far.pop();
                    }
                }
            }
        }

        let (mut palindromes_so_far, mut res) = (Vec::default(), Vec::default());
        rec(&s, &mut palindromes_so_far, &mut res);

        res
    }
}

fn main() {
    let test_cases = ["aab"];

    for s in test_cases {
        println!("{:#?}", Solution::partition(s.to_owned()));
    }
}
