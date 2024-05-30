// https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/description
pub struct Solution;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let prefix_xor = std::iter::once(0)
            .chain(arr.iter().scan(0, |state, &x| {
                *state ^= x;
                Some(*state)
            }))
            .collect::<Vec<_>>();

        let mut answer = 0;
        for i in 0..arr.len() {
            for k in i..arr.len() {
                for j in i + 1..=k {
                    if Solution::are_xors_eqal(i, j, k, &prefix_xor) {
                        answer += 1;
                    }
                }
            }
        }
        answer
    }

    fn are_xors_eqal(i: usize, j: usize, k: usize, xor_prefix: &[i32]) -> bool {
        let a = xor_prefix[i] ^ xor_prefix[j];
        let b = xor_prefix[j] ^ xor_prefix[k + 1];
        a == b
    }
}

fn main() {
    let test_cases = [vec![2, 3, 1, 6, 7], vec![1, 1, 1, 1, 1]];

    for arr in test_cases {
        println!("{}", Solution::count_triplets(arr));
    }
}
