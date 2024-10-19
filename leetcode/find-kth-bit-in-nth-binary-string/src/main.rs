// https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/description/
pub struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mut s = vec!['0'];

        fn invert(s: &[char]) -> Vec<char> {
            s.iter()
                .map(|&c| if c == '0' { '1' } else { '0' })
                .collect::<Vec<_>>()
        }

        for _ in 1..n {
            s.extend(std::iter::once('1').chain(invert(&s).into_iter().rev()));
        }

        s[(k - 1) as usize]
    }
}

fn main() {
    let test_cases = [(3, 1), (4, 11)];

    for (n, k) in test_cases {
        println!("{}", Solution::find_kth_bit(n, k));
    }
}
