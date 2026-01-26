// https://leetcode.com/problems/minimum-absolute-difference/description/?envType=daily-question&envId=2026-01-26
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        arr.windows(2)
            .fold(
                (i32::MAX, vec![]),
                |(min_diff, mut pairs), window| match (window[1] - window[0]).cmp(&min_diff) {
                    std::cmp::Ordering::Equal => {
                        pairs.push(window.to_vec());
                        (min_diff, pairs)
                    }
                    std::cmp::Ordering::Less => {
                        pairs.clear();
                        pairs.push(window.to_vec());
                        (window[1] - window[0], pairs)
                    }
                    std::cmp::Ordering::Greater => (min_diff, pairs),
                },
            )
            .1
    }
}

fn main() {
    println!("Hello, world!");
}
