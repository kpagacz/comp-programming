// https://leetcode.com/problems/count-square-sum-triples/description/?envType=daily-question&envId=2025-12-08
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut count = 0;
        for a in 1..=n {
            for b in 1..=n {
                let c_sq = a * a + b * b;
                if let Some(c) = Solution::perfect_square(c_sq)
                    && c <= n
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn perfect_square(n: i32) -> Option<i32> {
        let sq = (n as f32).sqrt().round() as i32;
        if sq * sq == n { Some(sq) } else { None }
    }
}

fn main() {
    println!("Hello, world!");
}
