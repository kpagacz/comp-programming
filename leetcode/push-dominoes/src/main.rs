// https://leetcode.com/problems/push-dominoes/description/?envType=daily-question&envId=2025-05-02
pub struct Solution;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut ds = dominoes.into_bytes();
        let mut left = 0;
        let mut right = 0;

        while left < ds.len() {
            while right + 1 < ds.len() && ds[right] == b'.' {
                right += 1;
            }

            match (ds[left], ds[right]) {
                (b'R', b'L') => {
                    let half = (right - left + 1) / 2;
                    ds[left..left + half].fill(b'R');
                    ds[right - half + 1..right].fill(b'L');
                }
                (b'R', _) => {
                    ds[left..=right].fill(b'R');
                }
                (_, b'L') => {
                    ds[left..=right].fill(b'L');
                }
                _ => {}
            }
            left = right;
            right += 1;
        }

        String::from_utf8(ds).unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
