// https://leetcode.com/problems/k-th-symbol-in-grammar/description/
pub struct Solution {}

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = format!("{:0n$b}", k - 1);
        let mut digit = 0;
        k.bytes().for_each(|c| {
            if c == b'1' {
                digit = 1 - digit
            }
        });
        digit
    }
    pub fn kth_grammar_rec(n: i32, k: i32) -> i32 {
        fn rec(root: i32, n: i32, k: i32) -> i32 {
            if n == 0 {
                return root;
            }
            if ((k - 1) >> (n - 1) & 1) == 1 {
                rec(1 - root, n - 1, k)
            } else {
                rec(root, n - 1, k)
            }
        }
        rec(0, n, k)
    }

    pub fn kth_grammar_just_count_the_flips(_: i32, k: i32) -> i32 {
        let actual_index = k - 1;
        (0..32).map(|pos| actual_index >> pos & 1).sum::<i32>() % 2
    }
}
fn main() {
    let test_cases = vec![(1, 1), (2, 1), (2, 2), (3, 1), (3, 2), (3, 3), (3, 4)];
    for (n, k) in test_cases {
        println!("{n} {k} : {}", Solution::kth_grammar(n, k));
        println!("rec: {}", Solution::kth_grammar_rec(n, k));
    }
}
