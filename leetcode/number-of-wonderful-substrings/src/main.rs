// https://leetcode.com/problems/number-of-wonderful-substrings/description/
struct Solution;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut prefixes = [0; 2048];
        prefixes[0] = 1;
        let (mut answer, mut bitmask) = (0i64, 0usize);
        word.as_bytes().iter().for_each(|c| {
            let c = c - b'a';
            bitmask ^= 1 << c;
            answer += prefixes[bitmask];
            prefixes[bitmask] += 1;

            for i in 0..10 {
                answer += prefixes[bitmask ^ 1 << i];
            }
        });

        answer
    }
}

fn main() {
    let test_cases = ["aba", "aabb"];
    for word in test_cases {
        println!("{}", Solution::wonderful_substrings(word.to_string()));
    }
}
